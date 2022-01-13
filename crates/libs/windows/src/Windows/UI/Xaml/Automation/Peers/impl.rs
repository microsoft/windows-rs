#[cfg(feature = "implement_exclusive")]
pub trait IAppBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IAppBarAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::AppBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBarAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IAppBarAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarAutomationPeerFactoryVtbl {
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
pub trait IAppBarButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarButtonAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarButtonAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IAppBarButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::AppBarButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBarButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IAppBarButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarButtonAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarButtonAutomationPeerFactoryVtbl {
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
pub trait IAppBarToggleButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarToggleButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarToggleButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarToggleButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarToggleButtonAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarToggleButtonAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarToggleButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarToggleButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IAppBarToggleButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::AppBarToggleButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarToggleButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBarToggleButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarToggleButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IAppBarToggleButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarToggleButtonAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarToggleButtonAutomationPeerFactoryVtbl {
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
pub trait IAutoSuggestBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutoSuggestBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutoSuggestBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAutoSuggestBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutoSuggestBoxAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutoSuggestBoxAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutoSuggestBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutoSuggestBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IAutoSuggestBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::AutoSuggestBox>) -> ::windows::core::Result<AutoSuggestBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutoSuggestBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutoSuggestBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IAutoSuggestBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutoSuggestBoxAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutoSuggestBoxAutomationPeerFactoryVtbl {
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
pub trait IAutomationPeerImpl: Sized {
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
impl IAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerVtbl {
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
pub trait IAutomationPeer2Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer2";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer2Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer2, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAutomationPeer3Impl: Sized {
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
impl IAutomationPeer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer3Vtbl {
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
pub trait IAutomationPeer4Impl: Sized {
    fn GetLandmarkType(&mut self) -> ::windows::core::Result<AutomationLandmarkType>;
    fn GetLocalizedLandmarkType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer4 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer4";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer4Vtbl {
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
pub trait IAutomationPeer5Impl: Sized {
    fn IsPeripheral(&mut self) -> ::windows::core::Result<bool>;
    fn IsDataValidForForm(&mut self) -> ::windows::core::Result<bool>;
    fn GetFullDescription(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer5 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer5";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer5Vtbl {
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
pub trait IAutomationPeer6Impl: Sized {
    fn GetCulture(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer6 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer6";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer6Vtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer6, BASE_OFFSET>(), GetCulture: GetCulture::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer7Impl: Sized {
    fn RaiseNotificationEvent(&mut self, notificationkind: AutomationNotificationKind, notificationprocessing: AutomationNotificationProcessing, displaystring: &::windows::core::HSTRING, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer7 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer7";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer7Vtbl {
        unsafe extern "system" fn RaiseNotificationEvent<Impl: IAutomationPeer7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationkind: AutomationNotificationKind, notificationprocessing: AutomationNotificationProcessing, displaystring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IAutomationPeer8Impl: Sized {
    fn GetHeadingLevel(&mut self) -> ::windows::core::Result<AutomationHeadingLevel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer8 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer8";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer8Vtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer8, BASE_OFFSET>(), GetHeadingLevel: GetHeadingLevel::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer9Impl: Sized {
    fn IsDialog(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer9 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer9";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeer9Vtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationPeer9, BASE_OFFSET>(), IsDialog: IsDialog::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationPeer9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerAnnotationImpl: Sized {
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
impl IAutomationPeerAnnotationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerAnnotationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerAnnotationVtbl {
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
pub trait IAutomationPeerAnnotationFactoryImpl: Sized {
    fn CreateInstance(&mut self, r#type: super::AnnotationType) -> ::windows::core::Result<AutomationPeerAnnotation>;
    fn CreateWithPeerParameter(&mut self, r#type: super::AnnotationType, peer: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<AutomationPeerAnnotation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerAnnotationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerAnnotationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerAnnotationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerAnnotationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerAnnotationFactoryVtbl {
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
pub trait IAutomationPeerAnnotationStaticsImpl: Sized {
    fn TypeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PeerProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerAnnotationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerAnnotationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerAnnotationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerAnnotationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerAnnotationStaticsVtbl {
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
pub trait IAutomationPeerFactoryImpl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerFactoryVtbl {
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
pub trait IAutomationPeerOverridesImpl: Sized {
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
impl IAutomationPeerOverridesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverridesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverridesVtbl {
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
pub trait IAutomationPeerOverrides2Impl: Sized {
    fn ShowContextMenuCore(&mut self) -> ::windows::core::Result<()>;
    fn GetControlledPeersCore(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AutomationPeer>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAutomationPeerOverrides2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides2Vtbl {
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
pub trait IAutomationPeerOverrides3Impl: Sized {
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
impl IAutomationPeerOverrides3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides3Vtbl {
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
pub trait IAutomationPeerOverrides4Impl: Sized {
    fn GetLandmarkTypeCore(&mut self) -> ::windows::core::Result<AutomationLandmarkType>;
    fn GetLocalizedLandmarkTypeCore(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides4 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides4";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides4Vtbl {
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
pub trait IAutomationPeerOverrides5Impl: Sized {
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
impl IAutomationPeerOverrides5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides5Vtbl {
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
pub trait IAutomationPeerOverrides6Impl: Sized {
    fn GetCultureCore(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides6 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides6";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides6Vtbl {
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
pub trait IAutomationPeerOverrides8Impl: Sized {
    fn GetHeadingLevelCore(&mut self) -> ::windows::core::Result<AutomationHeadingLevel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides8 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides8";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides8Vtbl {
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
pub trait IAutomationPeerOverrides9Impl: Sized {
    fn IsDialogCore(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides9 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides9";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides9Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerOverrides9Vtbl {
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
pub trait IAutomationPeerProtectedImpl: Sized {
    fn PeerFromProvider(&mut self, provider: &::core::option::Option<super::Provider::IRawElementProviderSimple>) -> ::windows::core::Result<AutomationPeer>;
    fn ProviderFromPeer(&mut self, peer: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<super::Provider::IRawElementProviderSimple>;
}
#[cfg(all(feature = "UI_Xaml_Automation_Provider", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAutomationPeerProtected {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerProtected";
}
#[cfg(all(feature = "UI_Xaml_Automation_Provider", feature = "implement_exclusive"))]
impl IAutomationPeerProtectedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerProtectedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerProtectedVtbl {
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
pub trait IAutomationPeerStaticsImpl: Sized {
    fn ListenerExists(&mut self, eventid: AutomationEvents) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerStaticsVtbl {
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
pub trait IAutomationPeerStatics3Impl: Sized {
    fn GenerateRawElementProviderRuntimeId(&mut self) -> ::windows::core::Result<RawElementProviderRuntimeId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationPeerStatics3Vtbl {
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
pub trait IButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Button>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonAutomationPeerFactoryVtbl {
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
pub trait IButtonBaseAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonBaseAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IButtonBaseAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonBaseAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonBaseAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonBaseAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IButtonBaseAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IButtonBaseAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IButtonBaseAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::ButtonBase>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ButtonBaseAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IButtonBaseAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IButtonBaseAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IButtonBaseAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonBaseAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonBaseAutomationPeerFactoryVtbl {
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
pub trait ICalendarDatePickerAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICalendarDatePickerAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICalendarDatePickerAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ICalendarDatePickerAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICalendarDatePickerAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICalendarDatePickerAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICalendarDatePickerAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICalendarDatePickerAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ICalendarDatePickerAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::CalendarDatePicker>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CalendarDatePickerAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICalendarDatePickerAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICalendarDatePickerAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ICalendarDatePickerAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICalendarDatePickerAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICalendarDatePickerAutomationPeerFactoryVtbl {
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
pub trait ICaptureElementAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICaptureElementAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICaptureElementAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ICaptureElementAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICaptureElementAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICaptureElementAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICaptureElementAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICaptureElementAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ICaptureElementAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::CaptureElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CaptureElementAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICaptureElementAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICaptureElementAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ICaptureElementAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICaptureElementAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICaptureElementAutomationPeerFactoryVtbl {
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
pub trait ICheckBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICheckBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICheckBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ICheckBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICheckBoxAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICheckBoxAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICheckBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICheckBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ICheckBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::CheckBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CheckBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICheckBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICheckBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ICheckBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICheckBoxAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICheckBoxAutomationPeerFactoryVtbl {
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
pub trait IColorPickerSliderAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPickerSliderAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IColorPickerSliderAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPickerSliderAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPickerSliderAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorPickerSliderAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IColorPickerSliderAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorPickerSliderAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IColorPickerSliderAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::ColorPickerSlider>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorPickerSliderAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorPickerSliderAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IColorPickerSliderAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IColorPickerSliderAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPickerSliderAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorPickerSliderAutomationPeerFactoryVtbl {
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
pub trait IColorSpectrumAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorSpectrumAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IColorSpectrumAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IColorSpectrumAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorSpectrumAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorSpectrumAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IColorSpectrumAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorSpectrumAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IColorSpectrumAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::ColorSpectrum>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorSpectrumAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorSpectrumAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IColorSpectrumAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IColorSpectrumAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorSpectrumAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorSpectrumAutomationPeerFactoryVtbl {
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
pub trait IComboBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IComboBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ComboBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IComboBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IComboBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxAutomationPeerFactoryVtbl {
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
pub trait IComboBoxItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IComboBoxItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ComboBoxItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IComboBoxItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IComboBoxItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxItemAutomationPeerFactoryVtbl {
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
pub trait IComboBoxItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxItemDataAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxItemDataAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ComboBoxAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxItemDataAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxItemDataAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxItemDataAutomationPeerFactoryVtbl {
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
pub trait IDatePickerAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatePickerAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IDatePickerAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IDatePickerAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatePickerAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatePickerAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDatePickerAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatePickerAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IDatePickerAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::DatePicker>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DatePickerAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDatePickerAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IDatePickerAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IDatePickerAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatePickerAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatePickerAutomationPeerFactoryVtbl {
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
pub trait IDatePickerFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatePickerFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IDatePickerFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IDatePickerFlyoutPresenterAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatePickerFlyoutPresenterAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatePickerFlyoutPresenterAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDatePickerFlyoutPresenterAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatePickerFlyoutPresenterAutomationPeer as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlipViewAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFlipViewAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlipViewAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IFlipViewAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::FlipView>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlipViewAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IFlipViewAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlipViewAutomationPeerFactoryVtbl {
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
pub trait IFlipViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlipViewItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFlipViewItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlipViewItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IFlipViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::FlipViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlipViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IFlipViewItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlipViewItemAutomationPeerFactoryVtbl {
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
pub trait IFlipViewItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewItemDataAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlipViewItemDataAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFlipViewItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlipViewItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<FlipViewAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewItemDataAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewItemDataAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlipViewItemDataAutomationPeerFactoryVtbl {
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
pub trait IFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutPresenterAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutPresenterAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutPresenterAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutPresenterAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutPresenterAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IFlyoutPresenterAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::FlyoutPresenter>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlyoutPresenterAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlyoutPresenterAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlyoutPresenterAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IFlyoutPresenterAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutPresenterAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutPresenterAutomationPeerFactoryVtbl {
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
pub trait IFrameworkElementAutomationPeerImpl: Sized {
    fn Owner(&mut self) -> ::windows::core::Result<super::super::UIElement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFrameworkElementAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementAutomationPeerVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementAutomationPeer, BASE_OFFSET>(), Owner: Owner::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::FrameworkElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FrameworkElementAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFrameworkElementAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementAutomationPeerFactoryVtbl {
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
pub trait IFrameworkElementAutomationPeerStaticsImpl: Sized {
    fn FromElement(&mut self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<AutomationPeer>;
    fn CreatePeerForElement(&mut self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<AutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementAutomationPeerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFrameworkElementAutomationPeerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementAutomationPeerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementAutomationPeerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementAutomationPeerStaticsVtbl {
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
pub trait IGridViewAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IGridViewAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::GridView>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGridViewAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IGridViewAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewAutomationPeerFactoryVtbl {
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
pub trait IGridViewHeaderItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewHeaderItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewHeaderItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewHeaderItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewHeaderItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewHeaderItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewHeaderItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewHeaderItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IGridViewHeaderItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::GridViewHeaderItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewHeaderItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGridViewHeaderItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewHeaderItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IGridViewHeaderItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewHeaderItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewHeaderItemAutomationPeerFactoryVtbl {
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
pub trait IGridViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IGridViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::GridViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGridViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IGridViewItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemAutomationPeerFactoryVtbl {
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
pub trait IGridViewItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemDataAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemDataAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<GridViewAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemDataAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemDataAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemDataAutomationPeerFactoryVtbl {
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
pub trait IGroupItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGroupItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGroupItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGroupItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGroupItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGroupItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGroupItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGroupItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IGroupItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::GroupItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GroupItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGroupItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGroupItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IGroupItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGroupItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGroupItemAutomationPeerFactoryVtbl {
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
pub trait IHubAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHubAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHubAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IHubAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHubAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHubAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHubAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHubAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IHubAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Hub>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HubAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHubAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHubAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IHubAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHubAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHubAutomationPeerFactoryVtbl {
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
pub trait IHubSectionAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHubSectionAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHubSectionAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IHubSectionAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHubSectionAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHubSectionAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHubSectionAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHubSectionAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IHubSectionAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::HubSection>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HubSectionAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHubSectionAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHubSectionAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IHubSectionAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHubSectionAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHubSectionAutomationPeerFactoryVtbl {
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
pub trait IHyperlinkButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlinkButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHyperlinkButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlinkButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkButtonAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHyperlinkButtonAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHyperlinkButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHyperlinkButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IHyperlinkButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::HyperlinkButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HyperlinkButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHyperlinkButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHyperlinkButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IHyperlinkButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkButtonAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHyperlinkButtonAutomationPeerFactoryVtbl {
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
pub trait IImageAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IImageAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IImageAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IImageAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IImageAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IImageAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Image>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ImageAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IImageAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IImageAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IImageAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IImageAutomationPeerFactoryVtbl {
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
pub trait IInkToolbarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkToolbarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IInkToolbarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IInkToolbarAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkToolbarAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInkToolbarAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInkToolbarAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInkToolbarAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemAutomationPeerImpl: Sized {
    fn Item(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ItemsControlAutomationPeer(&mut self) -> ::windows::core::Result<ItemsControlAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemAutomationPeerVtbl {
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
pub trait IItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ItemsControlAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemAutomationPeerFactoryVtbl {
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
pub trait IItemsControlAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemsControlAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IItemsControlAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsControlAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IItemsControlAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemsControlAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlAutomationPeer2Impl: Sized {
    fn CreateItemAutomationPeer(&mut self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemsControlAutomationPeer2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeer2";
}
#[cfg(feature = "implement_exclusive")]
impl IItemsControlAutomationPeer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsControlAutomationPeer2Vtbl {
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
pub trait IItemsControlAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ItemsControl>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemsControlAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IItemsControlAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IItemsControlAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsControlAutomationPeerFactoryVtbl {
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
pub trait IItemsControlAutomationPeerOverrides2Impl: Sized {
    fn OnCreateItemAutomationPeer(&mut self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemsControlAutomationPeerOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeerOverrides2";
}
#[cfg(feature = "implement_exclusive")]
impl IItemsControlAutomationPeerOverrides2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeerOverrides2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsControlAutomationPeerOverrides2Vtbl {
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
pub trait IListBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListBoxAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListBoxAutomationPeerFactoryVtbl {
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
pub trait IListBoxItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListBoxItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListBoxItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListBoxItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListBoxItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListBoxItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListBoxItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListBoxItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListBoxItemAutomationPeerFactoryVtbl {
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
pub trait IListBoxItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxItemDataAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListBoxItemDataAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListBoxItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListBoxItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ListBoxAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxItemDataAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxItemDataAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListBoxItemDataAutomationPeerFactoryVtbl {
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
pub trait IListPickerFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListPickerFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListPickerFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListPickerFlyoutPresenterAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListPickerFlyoutPresenterAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListPickerFlyoutPresenterAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListPickerFlyoutPresenterAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListPickerFlyoutPresenterAutomationPeer as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListViewAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListView>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListViewAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewAutomationPeerFactoryVtbl {
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
pub trait IListViewBaseAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewBaseAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewBaseAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewBaseAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewBaseAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewBaseAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewBaseAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewBaseAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListViewBaseAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListViewBase>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewBaseAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewBaseAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewBaseAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListViewBaseAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewBaseAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewBaseAutomationPeerFactoryVtbl {
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
pub trait IListViewBaseHeaderItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewBaseHeaderItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewBaseHeaderItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewBaseHeaderItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewBaseHeaderItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewBaseHeaderItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewBaseHeaderItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewBaseHeaderItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListViewBaseHeaderItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListViewBaseHeaderItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewBaseHeaderItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewBaseHeaderItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewBaseHeaderItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListViewBaseHeaderItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewBaseHeaderItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewBaseHeaderItemAutomationPeerFactoryVtbl {
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
pub trait IListViewHeaderItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewHeaderItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewHeaderItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewHeaderItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewHeaderItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewHeaderItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewHeaderItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewHeaderItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListViewHeaderItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListViewHeaderItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewHeaderItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewHeaderItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewHeaderItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListViewHeaderItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewHeaderItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewHeaderItemAutomationPeerFactoryVtbl {
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
pub trait IListViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IListViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ListViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IListViewItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemAutomationPeerFactoryVtbl {
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
pub trait IListViewItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemDataAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemDataAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ListViewBaseAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemDataAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemDataAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemDataAutomationPeerFactoryVtbl {
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
pub trait ILoopingSelectorAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ILoopingSelectorAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoopingSelectorAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoopingSelectorAutomationPeer as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoopingSelectorItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoopingSelectorItemAutomationPeer as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorItemDataAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorItemDataAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoopingSelectorItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoopingSelectorItemDataAutomationPeer as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMapControlAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMapControlAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMapControlAutomationPeer as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaElementAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaElementAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaElementAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaElementAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMediaElementAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::MediaElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaElementAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaElementAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaElementAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMediaElementAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaElementAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaElementAutomationPeerFactoryVtbl {
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
pub trait IMediaPlayerElementAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlayerElementAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaPlayerElementAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlayerElementAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerElementAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerElementAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaPlayerElementAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaPlayerElementAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMediaPlayerElementAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::MediaPlayerElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaPlayerElementAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaPlayerElementAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaPlayerElementAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMediaPlayerElementAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerElementAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaPlayerElementAutomationPeerFactoryVtbl {
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
pub trait IMediaTransportControlsAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaTransportControlsAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaTransportControlsAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaTransportControlsAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTransportControlsAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTransportControlsAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaTransportControlsAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaTransportControlsAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMediaTransportControlsAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::MediaTransportControls>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaTransportControlsAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaTransportControlsAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaTransportControlsAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMediaTransportControlsAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTransportControlsAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaTransportControlsAutomationPeerFactoryVtbl {
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
pub trait IMenuBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuBarAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuBarAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuBarAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuBarAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuBarAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMenuBarAutomationPeerFactoryImpl: Sized {
    fn CreateInstance(&mut self, owner: &::core::option::Option<super::super::Controls::MenuBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuBarAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMenuBarAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuBarAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMenuBarAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuBarAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuBarAutomationPeerFactoryVtbl {
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
pub trait IMenuBarItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuBarItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuBarItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuBarItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuBarItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuBarItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuBarItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuBarItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMenuBarItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstance(&mut self, owner: &::core::option::Option<super::super::Controls::MenuBarItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuBarItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMenuBarItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuBarItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMenuBarItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuBarItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuBarItemAutomationPeerFactoryVtbl {
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
pub trait IMenuFlyoutItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuFlyoutItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuFlyoutItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuFlyoutItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuFlyoutItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuFlyoutItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuFlyoutItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMenuFlyoutItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::MenuFlyoutItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuFlyoutItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMenuFlyoutItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuFlyoutItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMenuFlyoutItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuFlyoutItemAutomationPeerFactoryVtbl {
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
pub trait IMenuFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuFlyoutPresenterAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutPresenterAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuFlyoutPresenterAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuFlyoutPresenterAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuFlyoutPresenterAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IMenuFlyoutPresenterAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::MenuFlyoutPresenter>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuFlyoutPresenterAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMenuFlyoutPresenterAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuFlyoutPresenterAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IMenuFlyoutPresenterAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutPresenterAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuFlyoutPresenterAutomationPeerFactoryVtbl {
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
pub trait INavigationViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.INavigationViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationViewItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationViewItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationViewItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationViewItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationViewItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait INavigationViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::NavigationViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationViewItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for INavigationViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.INavigationViewItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl INavigationViewItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationViewItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationViewItemAutomationPeerFactoryVtbl {
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
pub trait IPasswordBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPasswordBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPasswordBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPasswordBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPasswordBoxAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPasswordBoxAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPasswordBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPasswordBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IPasswordBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::PasswordBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PasswordBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPasswordBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPasswordBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IPasswordBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPasswordBoxAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPasswordBoxAutomationPeerFactoryVtbl {
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
pub trait IPersonPictureAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPersonPictureAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPersonPictureAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPersonPictureAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersonPictureAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersonPictureAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPersonPictureAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersonPictureAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IPersonPictureAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::PersonPicture>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PersonPictureAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPersonPictureAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPersonPictureAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IPersonPictureAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersonPictureAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersonPictureAutomationPeerFactoryVtbl {
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
pub trait IPickerFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPickerFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPickerFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPickerFlyoutPresenterAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutPresenterAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPickerFlyoutPresenterAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPickerFlyoutPresenterAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPickerFlyoutPresenterAutomationPeer as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IPivotAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Pivot>) -> ::windows::core::Result<PivotAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPivotAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IPivotAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotAutomationPeerFactoryVtbl {
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
pub trait IPivotItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IPivotItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::PivotItem>) -> ::windows::core::Result<PivotItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPivotItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IPivotItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotItemAutomationPeerFactoryVtbl {
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
pub trait IPivotItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotItemDataAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotItemDataAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotItemDataAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotItemDataAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<PivotAutomationPeer>) -> ::windows::core::Result<PivotItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotItemDataAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotItemDataAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotItemDataAutomationPeerFactoryVtbl {
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
pub trait IProgressBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProgressBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IProgressBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IProgressBarAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressBarAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressBarAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IProgressBarAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressBarAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IProgressBarAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ProgressBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ProgressBarAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProgressBarAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IProgressBarAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IProgressBarAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressBarAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressBarAutomationPeerFactoryVtbl {
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
pub trait IProgressRingAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProgressRingAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IProgressRingAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IProgressRingAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressRingAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressRingAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IProgressRingAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressRingAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IProgressRingAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ProgressRing>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ProgressRingAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProgressRingAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IProgressRingAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IProgressRingAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressRingAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressRingAutomationPeerFactoryVtbl {
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
pub trait IRadioButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadioButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRadioButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRadioButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioButtonAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadioButtonAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRadioButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadioButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IRadioButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::RadioButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RadioButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadioButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRadioButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IRadioButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioButtonAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadioButtonAutomationPeerFactoryVtbl {
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
pub trait IRangeBaseAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBaseAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRangeBaseAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeBaseAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeBaseAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IRangeBaseAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::RangeBase>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RangeBaseAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRangeBaseAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRangeBaseAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IRangeBaseAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseAutomationPeerFactoryVtbl {
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
pub trait IRatingControlAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRatingControlAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRatingControlAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRatingControlAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatingControlAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRatingControlAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRatingControlAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRatingControlAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IRatingControlAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::RatingControl>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RatingControlAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRatingControlAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRatingControlAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IRatingControlAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatingControlAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRatingControlAutomationPeerFactoryVtbl {
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
pub trait IRepeatButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepeatButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRepeatButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRepeatButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatButtonAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepeatButtonAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRepeatButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepeatButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IRepeatButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::RepeatButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RepeatButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRepeatButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRepeatButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IRepeatButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatButtonAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepeatButtonAutomationPeerFactoryVtbl {
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
pub trait IRichEditBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichEditBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichEditBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRichEditBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditBoxAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichEditBoxAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRichEditBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichEditBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IRichEditBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::RichEditBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichEditBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRichEditBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichEditBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IRichEditBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditBoxAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichEditBoxAutomationPeerFactoryVtbl {
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
pub trait IRichTextBlockAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichTextBlockAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichTextBlockAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRichTextBlockAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichTextBlockAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichTextBlockAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRichTextBlockAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichTextBlockAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IRichTextBlockAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::RichTextBlock>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichTextBlockAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRichTextBlockAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichTextBlockAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IRichTextBlockAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichTextBlockAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichTextBlockAutomationPeerFactoryVtbl {
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
pub trait IRichTextBlockOverflowAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichTextBlockOverflowAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichTextBlockOverflowAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRichTextBlockOverflowAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichTextBlockOverflowAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichTextBlockOverflowAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRichTextBlockOverflowAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRichTextBlockOverflowAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IRichTextBlockOverflowAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::RichTextBlockOverflow>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichTextBlockOverflowAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRichTextBlockOverflowAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichTextBlockOverflowAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IRichTextBlockOverflowAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichTextBlockOverflowAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRichTextBlockOverflowAutomationPeerFactoryVtbl {
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
pub trait IScrollBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IScrollBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollBarAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollBarAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollBarAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollBarAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollBarAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IScrollBarAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::ScrollBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ScrollBarAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScrollBarAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IScrollBarAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IScrollBarAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollBarAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollBarAutomationPeerFactoryVtbl {
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
pub trait IScrollViewerAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollViewerAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IScrollViewerAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollViewerAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollViewerAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollViewerAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollViewerAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollViewerAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IScrollViewerAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ScrollViewer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ScrollViewerAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScrollViewerAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IScrollViewerAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IScrollViewerAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollViewerAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollViewerAutomationPeerFactoryVtbl {
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
pub trait ISearchBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISearchBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISearchBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISearchBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchBoxAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchBoxAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISearchBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISearchBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ISearchBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::SearchBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SearchBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISearchBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISearchBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ISearchBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchBoxAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISearchBoxAutomationPeerFactoryVtbl {
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
pub trait ISelectorAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISelectorAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectorAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectorAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait ISelectorAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::Selector>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SelectorAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISelectorAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISelectorAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ISelectorAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorAutomationPeerFactoryVtbl {
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
pub trait ISelectorItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISelectorItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectorItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectorItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<SelectorAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SelectorItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISelectorItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorItemAutomationPeerFactoryVtbl {
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
pub trait ISemanticZoomAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISemanticZoomAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISemanticZoomAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISemanticZoomAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISemanticZoomAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISemanticZoomAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISemanticZoomAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ISemanticZoomAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::SemanticZoom>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SemanticZoomAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISemanticZoomAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISemanticZoomAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ISemanticZoomAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISemanticZoomAutomationPeerFactoryVtbl {
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
pub trait ISettingsFlyoutAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISettingsFlyoutAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISettingsFlyoutAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISettingsFlyoutAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsFlyoutAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsFlyoutAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISettingsFlyoutAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsFlyoutAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ISettingsFlyoutAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::SettingsFlyout>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SettingsFlyoutAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsFlyoutAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISettingsFlyoutAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ISettingsFlyoutAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsFlyoutAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsFlyoutAutomationPeerFactoryVtbl {
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
pub trait ISliderAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISliderAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISliderAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISliderAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISliderAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISliderAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISliderAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISliderAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ISliderAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Slider>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SliderAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISliderAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISliderAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ISliderAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISliderAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISliderAutomationPeerFactoryVtbl {
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
pub trait ITextBlockAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextBlockAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITextBlockAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITextBlockAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextBlockAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextBlockAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITextBlockAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextBlockAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ITextBlockAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::TextBlock>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TextBlockAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextBlockAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITextBlockAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ITextBlockAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextBlockAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextBlockAutomationPeerFactoryVtbl {
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
pub trait ITextBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITextBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITextBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextBoxAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextBoxAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITextBoxAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITextBoxAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ITextBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::TextBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TextBoxAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITextBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITextBoxAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ITextBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextBoxAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITextBoxAutomationPeerFactoryVtbl {
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
pub trait IThumbAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThumbAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IThumbAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IThumbAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThumbAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThumbAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IThumbAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThumbAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IThumbAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::Thumb>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ThumbAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IThumbAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IThumbAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IThumbAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThumbAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThumbAutomationPeerFactoryVtbl {
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
pub trait ITimePickerAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimePickerAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITimePickerAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITimePickerAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimePickerAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimePickerAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITimePickerAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimePickerAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ITimePickerAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::TimePicker>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TimePickerAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITimePickerAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITimePickerAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ITimePickerAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimePickerAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimePickerAutomationPeerFactoryVtbl {
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
pub trait ITimePickerFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimePickerFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITimePickerFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITimePickerFlyoutPresenterAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimePickerFlyoutPresenterAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimePickerFlyoutPresenterAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITimePickerFlyoutPresenterAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimePickerFlyoutPresenterAutomationPeer as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleButtonAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleButtonAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleButtonAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IToggleButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::Primitives::ToggleButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleButtonAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToggleButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleButtonAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IToggleButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleButtonAutomationPeerFactoryVtbl {
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
pub trait IToggleMenuFlyoutItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleMenuFlyoutItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleMenuFlyoutItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleMenuFlyoutItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleMenuFlyoutItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleMenuFlyoutItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleMenuFlyoutItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleMenuFlyoutItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IToggleMenuFlyoutItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ToggleMenuFlyoutItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleMenuFlyoutItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToggleMenuFlyoutItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleMenuFlyoutItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IToggleMenuFlyoutItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleMenuFlyoutItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleMenuFlyoutItemAutomationPeerFactoryVtbl {
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
pub trait IToggleSwitchAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleSwitchAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleSwitchAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleSwitchAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleSwitchAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleSwitchAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleSwitchAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IToggleSwitchAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::ToggleSwitch>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleSwitchAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToggleSwitchAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleSwitchAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IToggleSwitchAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleSwitchAutomationPeerFactoryVtbl {
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
pub trait ITreeViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITreeViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITreeViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITreeViewItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITreeViewItemAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITreeViewItemAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITreeViewItemAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITreeViewItemAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ITreeViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::TreeViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeViewItemAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITreeViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITreeViewItemAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ITreeViewItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITreeViewItemAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITreeViewItemAutomationPeerFactoryVtbl {
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
pub trait ITreeViewListAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITreeViewListAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITreeViewListAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITreeViewListAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITreeViewListAutomationPeerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITreeViewListAutomationPeerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITreeViewListAutomationPeer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITreeViewListAutomationPeer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait ITreeViewListAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&mut self, owner: &::core::option::Option<super::super::Controls::TreeViewList>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeViewListAutomationPeer>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITreeViewListAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITreeViewListAutomationPeerFactory";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ITreeViewListAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITreeViewListAutomationPeerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITreeViewListAutomationPeerFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITreeViewListAutomationPeerFactory, BASE_OFFSET>(),
            CreateInstanceWithOwner: CreateInstanceWithOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITreeViewListAutomationPeerFactory as ::windows::core::Interface>::IID
    }
}
