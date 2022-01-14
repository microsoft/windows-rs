#[cfg(feature = "implement_exclusive")]
pub trait IAppBarAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IAppBarAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::AppBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBarAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IAppBarAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IAppBarAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarButtonAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarButtonAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarButtonAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IAppBarButtonAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::AppBarButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBarButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IAppBarButtonAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarButtonAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarButtonAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IAppBarButtonAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarButtonAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarButtonAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButtonAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarToggleButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarToggleButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarToggleButtonAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarToggleButtonAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarToggleButtonAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarToggleButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarToggleButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IAppBarToggleButtonAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::AppBarToggleButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarToggleButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBarToggleButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarToggleButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IAppBarToggleButtonAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarToggleButtonAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarToggleButtonAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IAppBarToggleButtonAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarToggleButtonAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarToggleButtonAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutoSuggestBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutoSuggestBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAutoSuggestBoxAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutoSuggestBoxAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutoSuggestBoxAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutoSuggestBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutoSuggestBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IAutoSuggestBoxAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::AutoSuggestBox>) -> ::windows::core::Result<AutoSuggestBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutoSuggestBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutoSuggestBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IAutoSuggestBoxAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutoSuggestBoxAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutoSuggestBoxAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IAutoSuggestBoxAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutoSuggestBoxAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutoSuggestBoxAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAutomationPeer_Impl: Sized {
    fn EventsSource(&mut self) -> ::windows::core::Result<AutomationPeer>;
    fn SetEventsSource(&mut self, value: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<()>;
    fn GetPattern(&mut self, patterninterface: PatternInterface) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn RaiseAutomationEvent(&mut self, eventid: AutomationEvents) -> ::windows::core::Result<()>;
    fn RaisePropertyChangedEvent(&mut self, automationproperty: &::core::option::Option<super::AutomationProperty>, oldvalue: &::core::option::Option<::windows::core::IInspectable>, newvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetAcceleratorKey(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAccessKey(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAutomationControlType(&mut self) -> ::windows::core::Result<AutomationControlType>;
    fn GetAutomationId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetBoundingRectangle(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn GetChildren(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeer>>;
    fn GetClassName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetClickablePoint(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn GetHelpText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemStatus(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLabeledBy(&mut self) -> ::windows::core::Result<AutomationPeer>;
    fn GetLocalizedControlType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetOrientation(&mut self) -> ::windows::core::Result<AutomationOrientation>;
    fn HasKeyboardFocus(&mut self) -> ::windows::core::Result<bool>;
    fn IsContentElement(&mut self) -> ::windows::core::Result<bool>;
    fn IsControlElement(&mut self) -> ::windows::core::Result<bool>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsKeyboardFocusable(&mut self) -> ::windows::core::Result<bool>;
    fn IsOffscreen(&mut self) -> ::windows::core::Result<bool>;
    fn IsPassword(&mut self) -> ::windows::core::Result<bool>;
    fn IsRequiredForForm(&mut self) -> ::windows::core::Result<bool>;
    fn SetFocus(&mut self) -> ::windows::core::Result<()>;
    fn GetParent(&mut self) -> ::windows::core::Result<AutomationPeer>;
    fn InvalidatePeer(&mut self) -> ::windows::core::Result<()>;
    fn GetPeerFromPoint(&mut self, point: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<AutomationPeer>;
    fn GetLiveSetting(&mut self) -> ::windows::core::Result<AutomationLiveSetting>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer_Vtbl {
        unsafe extern "system" fn EventsSource<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEventsSource<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventsSource(&*(&value as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPattern<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patterninterface: PatternInterface, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RaiseAutomationEvent<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: AutomationEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RaiseAutomationEvent(eventid).into()
        }
        unsafe extern "system" fn RaisePropertyChangedEvent<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automationproperty: ::windows::core::RawPtr, oldvalue: *mut ::core::ffi::c_void, newvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RaisePropertyChangedEvent(
                    &*(&automationproperty as *const <super::AutomationProperty as ::windows::core::Abi>::Abi as *const <super::AutomationProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&oldvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&newvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn GetAcceleratorKey<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAccessKey<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAutomationControlType<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationControlType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAutomationId<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetBoundingRectangle<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetChildren<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetClassName<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetClickablePoint<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetHelpText<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetItemStatus<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetItemType<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLabeledBy<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLocalizedControlType<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetName<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetOrientation<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationOrientation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasKeyboardFocus<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsContentElement<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsControlElement<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabled<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsKeyboardFocusable<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsOffscreen<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPassword<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsRequiredForForm<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFocus<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocus().into()
        }
        unsafe extern "system" fn GetParent<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InvalidatePeer<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidatePeer().into()
        }
        unsafe extern "system" fn GetPeerFromPoint<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLiveSetting<Impl: IAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLiveSetting) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer, BASE_OFFSET>(),
            EventsSource: EventsSource::<Impl, IMPL_OFFSET>,
            SetEventsSource: SetEventsSource::<Impl, IMPL_OFFSET>,
            GetPattern: GetPattern::<Impl, IMPL_OFFSET>,
            RaiseAutomationEvent: RaiseAutomationEvent::<Impl, IMPL_OFFSET>,
            RaisePropertyChangedEvent: RaisePropertyChangedEvent::<Impl, IMPL_OFFSET>,
            GetAcceleratorKey: GetAcceleratorKey::<Impl, IMPL_OFFSET>,
            GetAccessKey: GetAccessKey::<Impl, IMPL_OFFSET>,
            GetAutomationControlType: GetAutomationControlType::<Impl, IMPL_OFFSET>,
            GetAutomationId: GetAutomationId::<Impl, IMPL_OFFSET>,
            GetBoundingRectangle: GetBoundingRectangle::<Impl, IMPL_OFFSET>,
            GetChildren: GetChildren::<Impl, IMPL_OFFSET>,
            GetClassName: GetClassName::<Impl, IMPL_OFFSET>,
            GetClickablePoint: GetClickablePoint::<Impl, IMPL_OFFSET>,
            GetHelpText: GetHelpText::<Impl, IMPL_OFFSET>,
            GetItemStatus: GetItemStatus::<Impl, IMPL_OFFSET>,
            GetItemType: GetItemType::<Impl, IMPL_OFFSET>,
            GetLabeledBy: GetLabeledBy::<Impl, IMPL_OFFSET>,
            GetLocalizedControlType: GetLocalizedControlType::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetOrientation: GetOrientation::<Impl, IMPL_OFFSET>,
            HasKeyboardFocus: HasKeyboardFocus::<Impl, IMPL_OFFSET>,
            IsContentElement: IsContentElement::<Impl, IMPL_OFFSET>,
            IsControlElement: IsControlElement::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            IsKeyboardFocusable: IsKeyboardFocusable::<Impl, IMPL_OFFSET>,
            IsOffscreen: IsOffscreen::<Impl, IMPL_OFFSET>,
            IsPassword: IsPassword::<Impl, IMPL_OFFSET>,
            IsRequiredForForm: IsRequiredForForm::<Impl, IMPL_OFFSET>,
            SetFocus: SetFocus::<Impl, IMPL_OFFSET>,
            GetParent: GetParent::<Impl, IMPL_OFFSET>,
            InvalidatePeer: InvalidatePeer::<Impl, IMPL_OFFSET>,
            GetPeerFromPoint: GetPeerFromPoint::<Impl, IMPL_OFFSET>,
            GetLiveSetting: GetLiveSetting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer2_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer2";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer2_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer2, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAutomationPeer3_Impl: Sized {
    fn Navigate(&mut self, direction: AutomationNavigationDirection) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetElementFromPoint(&mut self, pointinwindowcoordinates: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetFocusedElement(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ShowContextMenu(&mut self) -> ::windows::core::Result<()>;
    fn GetControlledPeers(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AutomationPeer>>;
    fn GetAnnotations(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeerAnnotation>>;
    fn SetParent(&mut self, peer: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<()>;
    fn RaiseTextEditTextChangedEvent(&mut self, automationtexteditchangetype: super::AutomationTextEditChangeType, changeddata: &::core::option::Option<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn GetPositionInSet(&mut self) -> ::windows::core::Result<i32>;
    fn GetSizeOfSet(&mut self) -> ::windows::core::Result<i32>;
    fn GetLevel(&mut self) -> ::windows::core::Result<i32>;
    fn RaiseStructureChangedEvent(&mut self, structurechangetype: AutomationStructureChangeType, child: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPeer3 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutomationPeer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer3_Vtbl {
        unsafe extern "system" fn Navigate<Impl: IAutomationPeer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetElementFromPoint<Impl: IAutomationPeer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointinwindowcoordinates: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFocusedElement<Impl: IAutomationPeer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowContextMenu<Impl: IAutomationPeer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        unsafe extern "system" fn GetControlledPeers<Impl: IAutomationPeer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAnnotations<Impl: IAutomationPeer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetParent<Impl: IAutomationPeer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParent(&*(&peer as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RaiseTextEditTextChangedEvent<Impl: IAutomationPeer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automationtexteditchangetype: super::AutomationTextEditChangeType, changeddata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RaiseTextEditTextChangedEvent(automationtexteditchangetype, &*(&changeddata as *const <super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPositionInSet<Impl: IAutomationPeer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSizeOfSet<Impl: IAutomationPeer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLevel<Impl: IAutomationPeer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RaiseStructureChangedEvent<Impl: IAutomationPeer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, structurechangetype: AutomationStructureChangeType, child: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RaiseStructureChangedEvent(structurechangetype, &*(&child as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer3, BASE_OFFSET>(),
            Navigate: Navigate::<Impl, IMPL_OFFSET>,
            GetElementFromPoint: GetElementFromPoint::<Impl, IMPL_OFFSET>,
            GetFocusedElement: GetFocusedElement::<Impl, IMPL_OFFSET>,
            ShowContextMenu: ShowContextMenu::<Impl, IMPL_OFFSET>,
            GetControlledPeers: GetControlledPeers::<Impl, IMPL_OFFSET>,
            GetAnnotations: GetAnnotations::<Impl, IMPL_OFFSET>,
            SetParent: SetParent::<Impl, IMPL_OFFSET>,
            RaiseTextEditTextChangedEvent: RaiseTextEditTextChangedEvent::<Impl, IMPL_OFFSET>,
            GetPositionInSet: GetPositionInSet::<Impl, IMPL_OFFSET>,
            GetSizeOfSet: GetSizeOfSet::<Impl, IMPL_OFFSET>,
            GetLevel: GetLevel::<Impl, IMPL_OFFSET>,
            RaiseStructureChangedEvent: RaiseStructureChangedEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer4_Impl: Sized {
    fn GetLandmarkType(&mut self) -> ::windows::core::Result<AutomationLandmarkType>;
    fn GetLocalizedLandmarkType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer4 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer4";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer4_Vtbl {
        unsafe extern "system" fn GetLandmarkType<Impl: IAutomationPeer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLandmarkType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLocalizedLandmarkType<Impl: IAutomationPeer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer4, BASE_OFFSET>(),
            GetLandmarkType: GetLandmarkType::<Impl, IMPL_OFFSET>,
            GetLocalizedLandmarkType: GetLocalizedLandmarkType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer5_Impl: Sized {
    fn IsPeripheral(&mut self) -> ::windows::core::Result<bool>;
    fn IsDataValidForForm(&mut self) -> ::windows::core::Result<bool>;
    fn GetFullDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer5 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer5";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer5_Vtbl {
        unsafe extern "system" fn IsPeripheral<Impl: IAutomationPeer5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDataValidForForm<Impl: IAutomationPeer5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFullDescription<Impl: IAutomationPeer5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer5, BASE_OFFSET>(),
            IsPeripheral: IsPeripheral::<Impl, IMPL_OFFSET>,
            IsDataValidForForm: IsDataValidForForm::<Impl, IMPL_OFFSET>,
            GetFullDescription: GetFullDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer6_Impl: Sized {
    fn GetCulture(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer6 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer6";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer6_Vtbl {
        unsafe extern "system" fn GetCulture<Impl: IAutomationPeer6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer6, BASE_OFFSET>(), GetCulture: GetCulture::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer7_Impl: Sized {
    fn RaiseNotificationEvent(&mut self, notificationkind: AutomationNotificationKind, notificationprocessing: AutomationNotificationProcessing, displaystring: &::windows::core::HSTRING, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer7 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer7";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer7_Vtbl {
        unsafe extern "system" fn RaiseNotificationEvent<Impl: IAutomationPeer7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationkind: AutomationNotificationKind, notificationprocessing: AutomationNotificationProcessing, displaystring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RaiseNotificationEvent(notificationkind, notificationprocessing, &*(&displaystring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&activityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer7, BASE_OFFSET>(),
            RaiseNotificationEvent: RaiseNotificationEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer8_Impl: Sized {
    fn GetHeadingLevel(&mut self) -> ::windows::core::Result<AutomationHeadingLevel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer8 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer8";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer8_Vtbl {
        unsafe extern "system" fn GetHeadingLevel<Impl: IAutomationPeer8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationHeadingLevel) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer8, BASE_OFFSET>(), GetHeadingLevel: GetHeadingLevel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer9_Impl: Sized {
    fn IsDialog(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer9 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer9";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer9_Vtbl {
        unsafe extern "system" fn IsDialog<Impl: IAutomationPeer9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer9, BASE_OFFSET>(), IsDialog: IsDialog::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerAnnotation_Impl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<super::AnnotationType>;
    fn SetType(&mut self, value: super::AnnotationType) -> ::windows::core::Result<()>;
    fn Peer(&mut self) -> ::windows::core::Result<AutomationPeer>;
    fn SetPeer(&mut self, value: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerAnnotation {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerAnnotation";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerAnnotation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerAnnotation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerAnnotation_Vtbl {
        unsafe extern "system" fn Type<Impl: IAutomationPeerAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::AnnotationType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetType<Impl: IAutomationPeerAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::AnnotationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn Peer<Impl: IAutomationPeerAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPeer<Impl: IAutomationPeerAnnotation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPeer(&*(&value as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerAnnotation, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            Peer: Peer::<Impl, IMPL_OFFSET>,
            SetPeer: SetPeer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerAnnotation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerAnnotationFactory_Impl: Sized {
    fn CreateInstance(&mut self, r#type: super::AnnotationType) -> ::windows::core::Result<AutomationPeerAnnotation>;
    fn CreateWithPeerParameter(&mut self, r#type: super::AnnotationType, peer: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<AutomationPeerAnnotation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerAnnotationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerAnnotationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerAnnotationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerAnnotationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerAnnotationFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAutomationPeerAnnotationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: super::AnnotationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateWithPeerParameter<Impl: IAutomationPeerAnnotationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: super::AnnotationType, peer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerAnnotationFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateWithPeerParameter: CreateWithPeerParameter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerAnnotationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerAnnotationStatics_Impl: Sized {
    fn TypeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PeerProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerAnnotationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerAnnotationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerAnnotationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerAnnotationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerAnnotationStatics_Vtbl {
        unsafe extern "system" fn TypeProperty<Impl: IAutomationPeerAnnotationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PeerProperty<Impl: IAutomationPeerAnnotationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerAnnotationStatics, BASE_OFFSET>(),
            TypeProperty: TypeProperty::<Impl, IMPL_OFFSET>,
            PeerProperty: PeerProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerAnnotationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAutomationPeerOverrides_Impl: Sized {
    fn GetPatternCore(&mut self, patterninterface: PatternInterface) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetAcceleratorKeyCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAccessKeyCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAutomationControlTypeCore(&mut self) -> ::windows::core::Result<AutomationControlType>;
    fn GetAutomationIdCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetBoundingRectangleCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn GetChildrenCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeer>>;
    fn GetClassNameCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetClickablePointCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn GetHelpTextCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemStatusCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemTypeCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLabeledByCore(&mut self) -> ::windows::core::Result<AutomationPeer>;
    fn GetLocalizedControlTypeCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNameCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetOrientationCore(&mut self) -> ::windows::core::Result<AutomationOrientation>;
    fn HasKeyboardFocusCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsContentElementCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsControlElementCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsEnabledCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsKeyboardFocusableCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsOffscreenCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsPasswordCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsRequiredForFormCore(&mut self) -> ::windows::core::Result<bool>;
    fn SetFocusCore(&mut self) -> ::windows::core::Result<()>;
    fn GetPeerFromPointCore(&mut self, point: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<AutomationPeer>;
    fn GetLiveSettingCore(&mut self) -> ::windows::core::Result<AutomationLiveSetting>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutomationPeerOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides_Vtbl {
        unsafe extern "system" fn GetPatternCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patterninterface: PatternInterface, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAcceleratorKeyCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAccessKeyCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAutomationControlTypeCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationControlType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAutomationIdCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetBoundingRectangleCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetChildrenCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetClassNameCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetClickablePointCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetHelpTextCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetItemStatusCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetItemTypeCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLabeledByCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLocalizedControlTypeCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetNameCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetOrientationCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationOrientation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasKeyboardFocusCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsContentElementCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsControlElementCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabledCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsKeyboardFocusableCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsOffscreenCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPasswordCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsRequiredForFormCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFocusCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusCore().into()
        }
        unsafe extern "system" fn GetPeerFromPointCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLiveSettingCore<Impl: IAutomationPeerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLiveSetting) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides, BASE_OFFSET>(),
            GetPatternCore: GetPatternCore::<Impl, IMPL_OFFSET>,
            GetAcceleratorKeyCore: GetAcceleratorKeyCore::<Impl, IMPL_OFFSET>,
            GetAccessKeyCore: GetAccessKeyCore::<Impl, IMPL_OFFSET>,
            GetAutomationControlTypeCore: GetAutomationControlTypeCore::<Impl, IMPL_OFFSET>,
            GetAutomationIdCore: GetAutomationIdCore::<Impl, IMPL_OFFSET>,
            GetBoundingRectangleCore: GetBoundingRectangleCore::<Impl, IMPL_OFFSET>,
            GetChildrenCore: GetChildrenCore::<Impl, IMPL_OFFSET>,
            GetClassNameCore: GetClassNameCore::<Impl, IMPL_OFFSET>,
            GetClickablePointCore: GetClickablePointCore::<Impl, IMPL_OFFSET>,
            GetHelpTextCore: GetHelpTextCore::<Impl, IMPL_OFFSET>,
            GetItemStatusCore: GetItemStatusCore::<Impl, IMPL_OFFSET>,
            GetItemTypeCore: GetItemTypeCore::<Impl, IMPL_OFFSET>,
            GetLabeledByCore: GetLabeledByCore::<Impl, IMPL_OFFSET>,
            GetLocalizedControlTypeCore: GetLocalizedControlTypeCore::<Impl, IMPL_OFFSET>,
            GetNameCore: GetNameCore::<Impl, IMPL_OFFSET>,
            GetOrientationCore: GetOrientationCore::<Impl, IMPL_OFFSET>,
            HasKeyboardFocusCore: HasKeyboardFocusCore::<Impl, IMPL_OFFSET>,
            IsContentElementCore: IsContentElementCore::<Impl, IMPL_OFFSET>,
            IsControlElementCore: IsControlElementCore::<Impl, IMPL_OFFSET>,
            IsEnabledCore: IsEnabledCore::<Impl, IMPL_OFFSET>,
            IsKeyboardFocusableCore: IsKeyboardFocusableCore::<Impl, IMPL_OFFSET>,
            IsOffscreenCore: IsOffscreenCore::<Impl, IMPL_OFFSET>,
            IsPasswordCore: IsPasswordCore::<Impl, IMPL_OFFSET>,
            IsRequiredForFormCore: IsRequiredForFormCore::<Impl, IMPL_OFFSET>,
            SetFocusCore: SetFocusCore::<Impl, IMPL_OFFSET>,
            GetPeerFromPointCore: GetPeerFromPointCore::<Impl, IMPL_OFFSET>,
            GetLiveSettingCore: GetLiveSettingCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAutomationPeerOverrides2_Impl: Sized {
    fn ShowContextMenuCore(&mut self) -> ::windows::core::Result<()>;
    fn GetControlledPeersCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AutomationPeer>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutomationPeerOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides2_Vtbl {
        unsafe extern "system" fn ShowContextMenuCore<Impl: IAutomationPeerOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContextMenuCore().into()
        }
        unsafe extern "system" fn GetControlledPeersCore<Impl: IAutomationPeerOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides2, BASE_OFFSET>(),
            ShowContextMenuCore: ShowContextMenuCore::<Impl, IMPL_OFFSET>,
            GetControlledPeersCore: GetControlledPeersCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAutomationPeerOverrides3_Impl: Sized {
    fn NavigateCore(&mut self, direction: AutomationNavigationDirection) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetElementFromPointCore(&mut self, pointinwindowcoordinates: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetFocusedElementCore(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetAnnotationsCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeerAnnotation>>;
    fn GetPositionInSetCore(&mut self) -> ::windows::core::Result<i32>;
    fn GetSizeOfSetCore(&mut self) -> ::windows::core::Result<i32>;
    fn GetLevelCore(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides3 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutomationPeerOverrides3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides3_Vtbl {
        unsafe extern "system" fn NavigateCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetElementFromPointCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointinwindowcoordinates: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFocusedElementCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAnnotationsCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPositionInSetCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSizeOfSetCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLevelCore<Impl: IAutomationPeerOverrides3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides3, BASE_OFFSET>(),
            NavigateCore: NavigateCore::<Impl, IMPL_OFFSET>,
            GetElementFromPointCore: GetElementFromPointCore::<Impl, IMPL_OFFSET>,
            GetFocusedElementCore: GetFocusedElementCore::<Impl, IMPL_OFFSET>,
            GetAnnotationsCore: GetAnnotationsCore::<Impl, IMPL_OFFSET>,
            GetPositionInSetCore: GetPositionInSetCore::<Impl, IMPL_OFFSET>,
            GetSizeOfSetCore: GetSizeOfSetCore::<Impl, IMPL_OFFSET>,
            GetLevelCore: GetLevelCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides4_Impl: Sized {
    fn GetLandmarkTypeCore(&mut self) -> ::windows::core::Result<AutomationLandmarkType>;
    fn GetLocalizedLandmarkTypeCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides4 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides4";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides4_Vtbl {
        unsafe extern "system" fn GetLandmarkTypeCore<Impl: IAutomationPeerOverrides4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLandmarkType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLocalizedLandmarkTypeCore<Impl: IAutomationPeerOverrides4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides4, BASE_OFFSET>(),
            GetLandmarkTypeCore: GetLandmarkTypeCore::<Impl, IMPL_OFFSET>,
            GetLocalizedLandmarkTypeCore: GetLocalizedLandmarkTypeCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAutomationPeerOverrides5_Impl: Sized {
    fn IsPeripheralCore(&mut self) -> ::windows::core::Result<bool>;
    fn IsDataValidForFormCore(&mut self) -> ::windows::core::Result<bool>;
    fn GetFullDescriptionCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDescribedByCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterable<AutomationPeer>>;
    fn GetFlowsToCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterable<AutomationPeer>>;
    fn GetFlowsFromCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterable<AutomationPeer>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides5 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides5";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutomationPeerOverrides5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides5_Vtbl {
        unsafe extern "system" fn IsPeripheralCore<Impl: IAutomationPeerOverrides5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDataValidForFormCore<Impl: IAutomationPeerOverrides5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFullDescriptionCore<Impl: IAutomationPeerOverrides5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDescribedByCore<Impl: IAutomationPeerOverrides5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFlowsToCore<Impl: IAutomationPeerOverrides5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFlowsFromCore<Impl: IAutomationPeerOverrides5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides5, BASE_OFFSET>(),
            IsPeripheralCore: IsPeripheralCore::<Impl, IMPL_OFFSET>,
            IsDataValidForFormCore: IsDataValidForFormCore::<Impl, IMPL_OFFSET>,
            GetFullDescriptionCore: GetFullDescriptionCore::<Impl, IMPL_OFFSET>,
            GetDescribedByCore: GetDescribedByCore::<Impl, IMPL_OFFSET>,
            GetFlowsToCore: GetFlowsToCore::<Impl, IMPL_OFFSET>,
            GetFlowsFromCore: GetFlowsFromCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides6_Impl: Sized {
    fn GetCultureCore(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides6 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides6";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides6_Vtbl {
        unsafe extern "system" fn GetCultureCore<Impl: IAutomationPeerOverrides6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides6, BASE_OFFSET>(),
            GetCultureCore: GetCultureCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides8_Impl: Sized {
    fn GetHeadingLevelCore(&mut self) -> ::windows::core::Result<AutomationHeadingLevel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides8 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides8";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides8_Vtbl {
        unsafe extern "system" fn GetHeadingLevelCore<Impl: IAutomationPeerOverrides8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationHeadingLevel) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides8, BASE_OFFSET>(),
            GetHeadingLevelCore: GetHeadingLevelCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides9_Impl: Sized {
    fn IsDialogCore(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides9 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides9";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides9_Vtbl {
        unsafe extern "system" fn IsDialogCore<Impl: IAutomationPeerOverrides9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerOverrides9, BASE_OFFSET>(),
            IsDialogCore: IsDialogCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerOverrides9 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Automation_Provider", feature = "implement_exclusive"))]
pub trait IAutomationPeerProtected_Impl: Sized {
    fn PeerFromProvider(&mut self, provider: &::core::option::Option<super::Provider::IRawElementProviderSimple>) -> ::windows::core::Result<AutomationPeer>;
    fn ProviderFromPeer(&mut self, peer: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<super::Provider::IRawElementProviderSimple>;
}
#[cfg(all(feature = "UI_Xaml_Automation_Provider", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPeerProtected {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerProtected";
}
#[cfg(all(feature = "UI_Xaml_Automation_Provider", feature = "implement_exclusive"))]
impl IAutomationPeerProtected_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerProtected_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerProtected_Vtbl {
        unsafe extern "system" fn PeerFromProvider<Impl: IAutomationPeerProtected_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProviderFromPeer<Impl: IAutomationPeerProtected_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerProtected, BASE_OFFSET>(),
            PeerFromProvider: PeerFromProvider::<Impl, IMPL_OFFSET>,
            ProviderFromPeer: ProviderFromPeer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerProtected as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerStatics_Impl: Sized {
    fn ListenerExists(&mut self, eventid: AutomationEvents) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerStatics_Vtbl {
        unsafe extern "system" fn ListenerExists<Impl: IAutomationPeerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: AutomationEvents, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerStatics, BASE_OFFSET>(),
            ListenerExists: ListenerExists::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerStatics3_Impl: Sized {
    fn GenerateRawElementProviderRuntimeId(&mut self) -> ::windows::core::Result<RawElementProviderRuntimeId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerStatics3_Vtbl {
        unsafe extern "system" fn GenerateRawElementProviderRuntimeId<Impl: IAutomationPeerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RawElementProviderRuntimeId) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeerStatics3, BASE_OFFSET>(),
            GenerateRawElementProviderRuntimeId: GenerateRawElementProviderRuntimeId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IButtonAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Button>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IButtonAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IButtonAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IButtonAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IButtonAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonBaseAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonBaseAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IButtonBaseAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonBaseAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonBaseAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonBaseAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IButtonBaseAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IButtonBaseAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IButtonBaseAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::ButtonBase>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ButtonBaseAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IButtonBaseAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IButtonBaseAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IButtonBaseAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonBaseAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonBaseAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IButtonBaseAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IButtonBaseAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IButtonBaseAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePickerAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICalendarDatePickerAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICalendarDatePickerAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ICalendarDatePickerAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICalendarDatePickerAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICalendarDatePickerAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICalendarDatePickerAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICalendarDatePickerAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ICalendarDatePickerAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::CalendarDatePicker>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CalendarDatePickerAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICalendarDatePickerAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICalendarDatePickerAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ICalendarDatePickerAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICalendarDatePickerAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICalendarDatePickerAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ICalendarDatePickerAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICalendarDatePickerAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICalendarDatePickerAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICaptureElementAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICaptureElementAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICaptureElementAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ICaptureElementAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICaptureElementAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICaptureElementAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICaptureElementAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICaptureElementAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ICaptureElementAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::CaptureElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CaptureElementAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICaptureElementAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICaptureElementAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ICaptureElementAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICaptureElementAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICaptureElementAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ICaptureElementAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICaptureElementAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICaptureElementAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICheckBoxAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICheckBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICheckBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ICheckBoxAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICheckBoxAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICheckBoxAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICheckBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICheckBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ICheckBoxAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::CheckBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CheckBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICheckBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICheckBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ICheckBoxAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICheckBoxAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICheckBoxAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ICheckBoxAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICheckBoxAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICheckBoxAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerSliderAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPickerSliderAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IColorPickerSliderAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPickerSliderAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPickerSliderAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorPickerSliderAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IColorPickerSliderAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorPickerSliderAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IColorPickerSliderAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::ColorPickerSlider>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorPickerSliderAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorPickerSliderAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IColorPickerSliderAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IColorPickerSliderAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPickerSliderAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorPickerSliderAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IColorPickerSliderAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorPickerSliderAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorPickerSliderAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorSpectrumAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorSpectrumAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IColorSpectrumAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IColorSpectrumAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorSpectrumAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorSpectrumAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IColorSpectrumAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorSpectrumAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IColorSpectrumAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::ColorSpectrum>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorSpectrumAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorSpectrumAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IColorSpectrumAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IColorSpectrumAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorSpectrumAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorSpectrumAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IColorSpectrumAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorSpectrumAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorSpectrumAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IComboBoxAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ComboBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IComboBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IComboBoxAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IComboBoxAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IComboBoxItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ComboBoxItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IComboBoxItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IComboBoxItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IComboBoxItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemDataAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxItemDataAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxItemDataAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxItemDataAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemDataAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ComboBoxAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxItemDataAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxItemDataAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxItemDataAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IComboBoxItemDataAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxItemDataAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithParentAndItem: CreateInstanceWithParentAndItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxItemDataAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatePickerAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IDatePickerAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IDatePickerAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatePickerAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatePickerAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDatePickerAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatePickerAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IDatePickerAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::DatePicker>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DatePickerAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDatePickerAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IDatePickerAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IDatePickerAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatePickerAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatePickerAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IDatePickerAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDatePickerAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatePickerAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFlyoutPresenterAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatePickerFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IDatePickerFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IDatePickerFlyoutPresenterAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatePickerFlyoutPresenterAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatePickerFlyoutPresenterAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDatePickerFlyoutPresenterAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatePickerFlyoutPresenterAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlipViewAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFlipViewAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlipViewAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IFlipViewAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::FlipView>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlipViewAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IFlipViewAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlipViewAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IFlipViewAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlipViewAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlipViewAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlipViewItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFlipViewItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlipViewItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IFlipViewItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::FlipViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlipViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IFlipViewItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlipViewItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IFlipViewItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlipViewItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlipViewItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemDataAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewItemDataAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewItemDataAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlipViewItemDataAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFlipViewItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlipViewItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemDataAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<FlipViewAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewItemDataAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewItemDataAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlipViewItemDataAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IFlipViewItemDataAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlipViewItemDataAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithParentAndItem: CreateInstanceWithParentAndItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlipViewItemDataAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutPresenterAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutPresenterAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutPresenterAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutPresenterAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutPresenterAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutPresenterAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IFlyoutPresenterAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::FlyoutPresenter>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlyoutPresenterAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlyoutPresenterAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlyoutPresenterAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IFlyoutPresenterAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutPresenterAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutPresenterAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IFlyoutPresenterAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutPresenterAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutPresenterAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementAutomationPeer_Impl: Sized {
    fn Owner(&mut self) -> ::windows::core::Result<super::super::UIElement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFrameworkElementAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementAutomationPeer_Vtbl {
        unsafe extern "system" fn Owner<Impl: IFrameworkElementAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementAutomationPeer, BASE_OFFSET>(), Owner: Owner::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::FrameworkElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FrameworkElementAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFrameworkElementAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IFrameworkElementAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementAutomationPeerStatics_Impl: Sized {
    fn FromElement(&mut self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<AutomationPeer>;
    fn CreatePeerForElement(&mut self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<AutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementAutomationPeerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFrameworkElementAutomationPeerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementAutomationPeerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementAutomationPeerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementAutomationPeerStatics_Vtbl {
        unsafe extern "system" fn FromElement<Impl: IFrameworkElementAutomationPeerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreatePeerForElement<Impl: IFrameworkElementAutomationPeerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementAutomationPeerStatics, BASE_OFFSET>(),
            FromElement: FromElement::<Impl, IMPL_OFFSET>,
            CreatePeerForElement: CreatePeerForElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementAutomationPeerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IGridViewAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::GridView>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGridViewAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IGridViewAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IGridViewAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewHeaderItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewHeaderItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewHeaderItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewHeaderItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewHeaderItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewHeaderItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewHeaderItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewHeaderItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IGridViewHeaderItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::GridViewHeaderItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewHeaderItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGridViewHeaderItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewHeaderItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IGridViewHeaderItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewHeaderItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewHeaderItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IGridViewHeaderItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewHeaderItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewHeaderItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IGridViewItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::GridViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGridViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IGridViewItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IGridViewItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemDataAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemDataAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemDataAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemDataAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemDataAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<GridViewAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemDataAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemDataAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemDataAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IGridViewItemDataAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewItemDataAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithParentAndItem: CreateInstanceWithParentAndItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewItemDataAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGroupItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGroupItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGroupItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGroupItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGroupItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGroupItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGroupItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IGroupItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::GroupItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GroupItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGroupItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGroupItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IGroupItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGroupItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGroupItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IGroupItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGroupItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGroupItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHubAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHubAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IHubAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHubAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHubAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHubAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHubAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IHubAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Hub>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HubAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHubAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHubAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IHubAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHubAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHubAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IHubAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHubAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHubAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubSectionAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHubSectionAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHubSectionAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IHubSectionAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHubSectionAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHubSectionAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHubSectionAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHubSectionAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IHubSectionAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::HubSection>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HubSectionAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHubSectionAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHubSectionAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IHubSectionAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHubSectionAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHubSectionAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IHubSectionAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHubSectionAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHubSectionAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkButtonAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlinkButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHyperlinkButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlinkButtonAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkButtonAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHyperlinkButtonAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHyperlinkButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHyperlinkButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IHyperlinkButtonAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::HyperlinkButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HyperlinkButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHyperlinkButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHyperlinkButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IHyperlinkButtonAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkButtonAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHyperlinkButtonAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IHyperlinkButtonAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHyperlinkButtonAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHyperlinkButtonAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IImageAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IImageAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IImageAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IImageAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Image>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ImageAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IImageAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IImageAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IImageAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IImageAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkToolbarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IInkToolbarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IInkToolbarAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkToolbarAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkToolbarAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInkToolbarAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkToolbarAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemAutomationPeer_Impl: Sized {
    fn Item(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ItemsControlAutomationPeer(&mut self) -> ::windows::core::Result<ItemsControlAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemAutomationPeer_Vtbl {
        unsafe extern "system" fn Item<Impl: IItemAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemsControlAutomationPeer<Impl: IItemAutomationPeer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemAutomationPeer, BASE_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            ItemsControlAutomationPeer: ItemsControlAutomationPeer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ItemsControlAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithParentAndItem: CreateInstanceWithParentAndItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemsControlAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IItemsControlAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsControlAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IItemsControlAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemsControlAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlAutomationPeer2_Impl: Sized {
    fn CreateItemAutomationPeer(&mut self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemsControlAutomationPeer2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeer2";
}
#[cfg(feature = "implement_exclusive")]
impl IItemsControlAutomationPeer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsControlAutomationPeer2_Vtbl {
        unsafe extern "system" fn CreateItemAutomationPeer<Impl: IItemsControlAutomationPeer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemsControlAutomationPeer2, BASE_OFFSET>(),
            CreateItemAutomationPeer: CreateItemAutomationPeer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemsControlAutomationPeer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IItemsControlAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ItemsControl>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemsControlAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IItemsControlAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IItemsControlAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsControlAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IItemsControlAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemsControlAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemsControlAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlAutomationPeerOverrides2_Impl: Sized {
    fn OnCreateItemAutomationPeer(&mut self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemsControlAutomationPeerOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeerOverrides2";
}
#[cfg(feature = "implement_exclusive")]
impl IItemsControlAutomationPeerOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeerOverrides2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsControlAutomationPeerOverrides2_Vtbl {
        unsafe extern "system" fn OnCreateItemAutomationPeer<Impl: IItemsControlAutomationPeerOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemsControlAutomationPeerOverrides2, BASE_OFFSET>(),
            OnCreateItemAutomationPeer: OnCreateItemAutomationPeer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemsControlAutomationPeerOverrides2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListBoxAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListBoxAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListBoxAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListBoxAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListBoxAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListBoxAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListBoxAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListBoxItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListBoxItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListBoxItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListBoxItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListBoxItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListBoxItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListBoxItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListBoxItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListBoxItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListBoxItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListBoxItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemDataAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxItemDataAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxItemDataAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListBoxItemDataAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListBoxItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListBoxItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemDataAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ListBoxAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxItemDataAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxItemDataAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListBoxItemDataAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IListBoxItemDataAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListBoxItemDataAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithParentAndItem: CreateInstanceWithParentAndItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListBoxItemDataAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListPickerFlyoutPresenterAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListPickerFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListPickerFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListPickerFlyoutPresenterAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListPickerFlyoutPresenterAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListPickerFlyoutPresenterAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListPickerFlyoutPresenterAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListPickerFlyoutPresenterAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListViewAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListView>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListViewAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewBaseAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewBaseAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewBaseAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewBaseAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewBaseAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewBaseAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewBaseAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListViewBaseAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListViewBase>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewBaseAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewBaseAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewBaseAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListViewBaseAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewBaseAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewBaseAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewBaseAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewBaseAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewBaseAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseHeaderItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewBaseHeaderItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewBaseHeaderItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewBaseHeaderItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewBaseHeaderItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewBaseHeaderItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewBaseHeaderItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewBaseHeaderItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListViewBaseHeaderItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListViewBaseHeaderItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewBaseHeaderItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewBaseHeaderItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewBaseHeaderItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListViewBaseHeaderItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewBaseHeaderItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewBaseHeaderItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewBaseHeaderItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewBaseHeaderItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewBaseHeaderItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewHeaderItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewHeaderItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewHeaderItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewHeaderItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewHeaderItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewHeaderItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewHeaderItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewHeaderItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListViewHeaderItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListViewHeaderItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewHeaderItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewHeaderItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewHeaderItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListViewHeaderItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewHeaderItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewHeaderItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewHeaderItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewHeaderItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewHeaderItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListViewItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListViewItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemDataAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemDataAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemDataAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemDataAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemDataAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ListViewBaseAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemDataAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemDataAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemDataAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IListViewItemDataAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemDataAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithParentAndItem: CreateInstanceWithParentAndItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemDataAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ILoopingSelectorAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoopingSelectorAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoopingSelectorAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ILoopingSelectorItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoopingSelectorItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoopingSelectorItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorItemDataAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ILoopingSelectorItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorItemDataAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorItemDataAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorItemDataAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoopingSelectorItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoopingSelectorItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMapControlAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaElementAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaElementAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaElementAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaElementAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaElementAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaElementAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaElementAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaElementAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMediaElementAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::MediaElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaElementAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaElementAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaElementAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMediaElementAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaElementAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaElementAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMediaElementAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaElementAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaElementAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerElementAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlayerElementAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaPlayerElementAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlayerElementAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerElementAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerElementAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayerElementAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerElementAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMediaPlayerElementAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::MediaPlayerElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaPlayerElementAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayerElementAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaPlayerElementAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMediaPlayerElementAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerElementAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerElementAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMediaPlayerElementAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayerElementAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerElementAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaTransportControlsAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaTransportControlsAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaTransportControlsAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTransportControlsAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTransportControlsAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaTransportControlsAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTransportControlsAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMediaTransportControlsAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::MediaTransportControls>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaTransportControlsAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaTransportControlsAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaTransportControlsAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMediaTransportControlsAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTransportControlsAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTransportControlsAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMediaTransportControlsAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaTransportControlsAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTransportControlsAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuBarAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuBarAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuBarAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuBarAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuBarAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMenuBarAutomationPeerFactory_Impl: Sized {
    fn CreateInstance(&mut self, owner: &::core::option::Option<super::super::Controls::MenuBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuBarAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMenuBarAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuBarAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMenuBarAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuBarAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuBarAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMenuBarAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuBarAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuBarAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuBarItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuBarItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuBarItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuBarItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuBarItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuBarItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuBarItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMenuBarItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstance(&mut self, owner: &::core::option::Option<super::super::Controls::MenuBarItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuBarItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMenuBarItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuBarItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMenuBarItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuBarItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuBarItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMenuBarItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuBarItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuBarItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuFlyoutItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuFlyoutItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuFlyoutItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuFlyoutItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuFlyoutItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuFlyoutItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMenuFlyoutItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::MenuFlyoutItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuFlyoutItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMenuFlyoutItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuFlyoutItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMenuFlyoutItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuFlyoutItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMenuFlyoutItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuFlyoutItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuFlyoutItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutPresenterAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuFlyoutPresenterAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutPresenterAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuFlyoutPresenterAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuFlyoutPresenterAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuFlyoutPresenterAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMenuFlyoutPresenterAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::MenuFlyoutPresenter>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuFlyoutPresenterAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMenuFlyoutPresenterAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuFlyoutPresenterAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMenuFlyoutPresenterAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutPresenterAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuFlyoutPresenterAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMenuFlyoutPresenterAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuFlyoutPresenterAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuFlyoutPresenterAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.INavigationViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationViewItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationViewItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationViewItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationViewItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationViewItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait INavigationViewItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::NavigationViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationViewItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INavigationViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.INavigationViewItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl INavigationViewItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationViewItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationViewItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: INavigationViewItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationViewItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationViewItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBoxAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPasswordBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPasswordBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPasswordBoxAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPasswordBoxAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPasswordBoxAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPasswordBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPasswordBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IPasswordBoxAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::PasswordBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PasswordBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPasswordBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPasswordBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IPasswordBoxAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPasswordBoxAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPasswordBoxAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IPasswordBoxAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPasswordBoxAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPasswordBoxAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPersonPictureAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPersonPictureAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPersonPictureAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPersonPictureAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersonPictureAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersonPictureAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPersonPictureAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersonPictureAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IPersonPictureAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::PersonPicture>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PersonPictureAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPersonPictureAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPersonPictureAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IPersonPictureAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersonPictureAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersonPictureAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IPersonPictureAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPersonPictureAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersonPictureAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutPresenterAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPickerFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPickerFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPickerFlyoutPresenterAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutPresenterAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPickerFlyoutPresenterAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPickerFlyoutPresenterAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPickerFlyoutPresenterAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IPivotAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Pivot>) -> ::windows::core::Result<PivotAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPivotAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IPivotAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IPivotAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IPivotItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::PivotItem>) -> ::windows::core::Result<PivotItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPivotItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IPivotItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IPivotItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemDataAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotItemDataAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotItemDataAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotItemDataAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemDataAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<PivotAutomationPeer>) -> ::windows::core::Result<PivotItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotItemDataAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotItemDataAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotItemDataAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IPivotItemDataAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotItemDataAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithParentAndItem: CreateInstanceWithParentAndItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotItemDataAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressBarAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProgressBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IProgressBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IProgressBarAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressBarAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressBarAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IProgressBarAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressBarAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IProgressBarAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ProgressBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ProgressBarAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProgressBarAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IProgressBarAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IProgressBarAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressBarAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressBarAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IProgressBarAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProgressBarAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressBarAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressRingAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProgressRingAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IProgressRingAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IProgressRingAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressRingAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressRingAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IProgressRingAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressRingAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IProgressRingAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ProgressRing>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ProgressRingAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProgressRingAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IProgressRingAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IProgressRingAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressRingAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressRingAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IProgressRingAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProgressRingAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressRingAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadioButtonAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadioButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRadioButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRadioButtonAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioButtonAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadioButtonAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRadioButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadioButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IRadioButtonAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::RadioButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RadioButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadioButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRadioButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IRadioButtonAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioButtonAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadioButtonAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRadioButtonAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadioButtonAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadioButtonAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBaseAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRangeBaseAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeBaseAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeBaseAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IRangeBaseAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::RangeBase>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RangeBaseAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRangeBaseAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRangeBaseAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IRangeBaseAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRangeBaseAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeBaseAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeBaseAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingControlAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRatingControlAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRatingControlAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRatingControlAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatingControlAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRatingControlAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRatingControlAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRatingControlAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IRatingControlAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::RatingControl>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RatingControlAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRatingControlAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRatingControlAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IRatingControlAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatingControlAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRatingControlAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRatingControlAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRatingControlAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRatingControlAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatButtonAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepeatButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRepeatButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRepeatButtonAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatButtonAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepeatButtonAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRepeatButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepeatButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IRepeatButtonAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::RepeatButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RepeatButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRepeatButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRepeatButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IRepeatButtonAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatButtonAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepeatButtonAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRepeatButtonAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRepeatButtonAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepeatButtonAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichEditBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichEditBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRichEditBoxAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditBoxAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichEditBoxAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRichEditBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichEditBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IRichEditBoxAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::RichEditBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichEditBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRichEditBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichEditBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IRichEditBoxAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditBoxAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichEditBoxAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRichEditBoxAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRichEditBoxAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichEditBoxAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichTextBlockAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichTextBlockAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRichTextBlockAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichTextBlockAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichTextBlockAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRichTextBlockAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichTextBlockAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IRichTextBlockAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::RichTextBlock>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichTextBlockAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRichTextBlockAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichTextBlockAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IRichTextBlockAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichTextBlockAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichTextBlockAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRichTextBlockAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRichTextBlockAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichTextBlockAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockOverflowAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichTextBlockOverflowAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichTextBlockOverflowAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRichTextBlockOverflowAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichTextBlockOverflowAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichTextBlockOverflowAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRichTextBlockOverflowAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichTextBlockOverflowAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IRichTextBlockOverflowAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::RichTextBlockOverflow>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichTextBlockOverflowAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRichTextBlockOverflowAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichTextBlockOverflowAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IRichTextBlockOverflowAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichTextBlockOverflowAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichTextBlockOverflowAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRichTextBlockOverflowAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRichTextBlockOverflowAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichTextBlockOverflowAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollBarAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IScrollBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollBarAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollBarAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollBarAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollBarAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollBarAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IScrollBarAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::ScrollBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ScrollBarAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScrollBarAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IScrollBarAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IScrollBarAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollBarAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollBarAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IScrollBarAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollBarAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollBarAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewerAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollViewerAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IScrollViewerAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollViewerAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollViewerAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollViewerAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollViewerAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollViewerAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IScrollViewerAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ScrollViewer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ScrollViewerAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScrollViewerAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IScrollViewerAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IScrollViewerAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollViewerAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollViewerAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IScrollViewerAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollViewerAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollViewerAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchBoxAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISearchBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISearchBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISearchBoxAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchBoxAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchBoxAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISearchBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ISearchBoxAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::SearchBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SearchBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISearchBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ISearchBoxAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchBoxAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchBoxAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISearchBoxAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISearchBoxAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchBoxAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISelectorAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectorAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectorAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait ISelectorAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::Selector>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SelectorAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISelectorAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISelectorAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ISelectorAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISelectorAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectorAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectorAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISelectorItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectorItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectorItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<SelectorAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SelectorItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISelectorItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: ISelectorItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectorItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithParentAndItem: CreateInstanceWithParentAndItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectorItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticZoomAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISemanticZoomAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISemanticZoomAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISemanticZoomAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISemanticZoomAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISemanticZoomAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISemanticZoomAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ISemanticZoomAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::SemanticZoom>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SemanticZoomAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISemanticZoomAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISemanticZoomAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ISemanticZoomAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISemanticZoomAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISemanticZoomAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISemanticZoomAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISemanticZoomAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsFlyoutAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISettingsFlyoutAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISettingsFlyoutAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISettingsFlyoutAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsFlyoutAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsFlyoutAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISettingsFlyoutAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsFlyoutAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ISettingsFlyoutAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::SettingsFlyout>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SettingsFlyoutAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsFlyoutAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISettingsFlyoutAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ISettingsFlyoutAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsFlyoutAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsFlyoutAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISettingsFlyoutAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISettingsFlyoutAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsFlyoutAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISliderAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISliderAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISliderAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISliderAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISliderAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISliderAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISliderAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISliderAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ISliderAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Slider>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SliderAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISliderAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISliderAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ISliderAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISliderAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISliderAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISliderAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISliderAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISliderAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlockAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextBlockAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITextBlockAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITextBlockAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextBlockAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextBlockAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITextBlockAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextBlockAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ITextBlockAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::TextBlock>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TextBlockAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextBlockAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITextBlockAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ITextBlockAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextBlockAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextBlockAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITextBlockAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextBlockAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextBlockAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITextBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITextBoxAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextBoxAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextBoxAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITextBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ITextBoxAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::TextBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TextBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITextBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ITextBoxAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextBoxAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextBoxAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITextBoxAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITextBoxAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextBoxAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThumbAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThumbAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IThumbAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IThumbAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThumbAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThumbAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IThumbAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThumbAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IThumbAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::Thumb>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ThumbAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IThumbAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IThumbAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IThumbAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThumbAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThumbAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IThumbAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IThumbAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThumbAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimePickerAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITimePickerAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITimePickerAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimePickerAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimePickerAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITimePickerAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimePickerAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ITimePickerAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::TimePicker>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TimePickerAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimePickerAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITimePickerAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ITimePickerAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimePickerAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimePickerAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITimePickerAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITimePickerAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimePickerAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerFlyoutPresenterAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimePickerFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITimePickerFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITimePickerFlyoutPresenterAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimePickerFlyoutPresenterAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimePickerFlyoutPresenterAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITimePickerFlyoutPresenterAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimePickerFlyoutPresenterAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleButtonAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleButtonAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IToggleButtonAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::ToggleButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToggleButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IToggleButtonAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleButtonAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IToggleButtonAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleButtonAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleButtonAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleMenuFlyoutItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleMenuFlyoutItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleMenuFlyoutItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleMenuFlyoutItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleMenuFlyoutItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleMenuFlyoutItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleMenuFlyoutItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleMenuFlyoutItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IToggleMenuFlyoutItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ToggleMenuFlyoutItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleMenuFlyoutItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToggleMenuFlyoutItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleMenuFlyoutItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IToggleMenuFlyoutItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleMenuFlyoutItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleMenuFlyoutItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IToggleMenuFlyoutItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleMenuFlyoutItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleMenuFlyoutItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSwitchAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleSwitchAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleSwitchAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleSwitchAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleSwitchAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleSwitchAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleSwitchAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IToggleSwitchAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ToggleSwitch>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleSwitchAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToggleSwitchAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleSwitchAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IToggleSwitchAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleSwitchAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IToggleSwitchAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleSwitchAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleSwitchAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITreeViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITreeViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITreeViewItemAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITreeViewItemAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITreeViewItemAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITreeViewItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITreeViewItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ITreeViewItemAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::TreeViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeViewItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITreeViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITreeViewItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ITreeViewItemAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITreeViewItemAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITreeViewItemAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITreeViewItemAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITreeViewItemAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITreeViewItemAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewListAutomationPeer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITreeViewListAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITreeViewListAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITreeViewListAutomationPeer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITreeViewListAutomationPeer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITreeViewListAutomationPeer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITreeViewListAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITreeViewListAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ITreeViewListAutomationPeerFactory_Impl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::TreeViewList>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeViewListAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITreeViewListAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITreeViewListAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ITreeViewListAutomationPeerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITreeViewListAutomationPeerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITreeViewListAutomationPeerFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITreeViewListAutomationPeerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITreeViewListAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITreeViewListAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
