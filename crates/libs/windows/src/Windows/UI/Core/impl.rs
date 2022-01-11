#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAcceleratorKeyEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn EventType(&self) -> ::windows::core::Result<CoreAcceleratorKeyEventType>;
    fn VirtualKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn KeyStatus(&self) -> ::windows::core::Result<CorePhysicalKeyStatus>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAcceleratorKeyEventArgs {
    const NAME: &'static str = "Windows.UI.Core.IAcceleratorKeyEventArgs";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAcceleratorKeyEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAcceleratorKeyEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAcceleratorKeyEventArgsVtbl {
        unsafe extern "system" fn EventType<Impl: IAcceleratorKeyEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreAcceleratorKeyEventType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VirtualKey<Impl: IAcceleratorKeyEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VirtualKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyStatus<Impl: IAcceleratorKeyEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CorePhysicalKeyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAcceleratorKeyEventArgs, BASE_OFFSET>(),
            EventType: EventType::<Impl, IMPL_OFFSET>,
            VirtualKey: VirtualKey::<Impl, IMPL_OFFSET>,
            KeyStatus: KeyStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAcceleratorKeyEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAcceleratorKeyEventArgs2Impl: Sized + ICoreWindowEventArgsImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAcceleratorKeyEventArgs2 {
    const NAME: &'static str = "Windows.UI.Core.IAcceleratorKeyEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IAcceleratorKeyEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAcceleratorKeyEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAcceleratorKeyEventArgs2Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IAcceleratorKeyEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAcceleratorKeyEventArgs2, BASE_OFFSET>(), DeviceId: DeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAcceleratorKeyEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationProviderRequestedEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn AutomationProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetAutomationProvider(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationProviderRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.IAutomationProviderRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationProviderRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationProviderRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationProviderRequestedEventArgsVtbl {
        unsafe extern "system" fn AutomationProvider<Impl: IAutomationProviderRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomationProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomationProvider<Impl: IAutomationProviderRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutomationProvider(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationProviderRequestedEventArgs, BASE_OFFSET>(),
            AutomationProvider: AutomationProvider::<Impl, IMPL_OFFSET>,
            SetAutomationProvider: SetAutomationProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationProviderRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackRequestedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBackRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.IBackRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBackRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackRequestedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IBackRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IBackRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackRequestedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICharacterReceivedEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn KeyCode(&self) -> ::windows::core::Result<u32>;
    fn KeyStatus(&self) -> ::windows::core::Result<CorePhysicalKeyStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICharacterReceivedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.ICharacterReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICharacterReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICharacterReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICharacterReceivedEventArgsVtbl {
        unsafe extern "system" fn KeyCode<Impl: ICharacterReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyStatus<Impl: ICharacterReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CorePhysicalKeyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICharacterReceivedEventArgs, BASE_OFFSET>(),
            KeyCode: KeyCode::<Impl, IMPL_OFFSET>,
            KeyStatus: KeyStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICharacterReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IClosestInteractiveBoundsRequestedEventArgsImpl: Sized {
    fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SearchBounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn ClosestInteractiveBounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetClosestInteractiveBounds(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IClosestInteractiveBoundsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.IClosestInteractiveBoundsRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IClosestInteractiveBoundsRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClosestInteractiveBoundsRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClosestInteractiveBoundsRequestedEventArgsVtbl {
        unsafe extern "system" fn PointerPosition<Impl: IClosestInteractiveBoundsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchBounds<Impl: IClosestInteractiveBoundsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClosestInteractiveBounds<Impl: IClosestInteractiveBoundsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosestInteractiveBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClosestInteractiveBounds<Impl: IClosestInteractiveBoundsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClosestInteractiveBounds(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IClosestInteractiveBoundsRequestedEventArgs, BASE_OFFSET>(),
            PointerPosition: PointerPosition::<Impl, IMPL_OFFSET>,
            SearchBounds: SearchBounds::<Impl, IMPL_OFFSET>,
            ClosestInteractiveBounds: ClosestInteractiveBounds::<Impl, IMPL_OFFSET>,
            SetClosestInteractiveBounds: SetClosestInteractiveBounds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClosestInteractiveBoundsRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ICoreAcceleratorKeysImpl: Sized {
    fn AcceleratorKeyActivated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAcceleratorKeyActivated(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICoreAcceleratorKeys {
    const NAME: &'static str = "Windows.UI.Core.ICoreAcceleratorKeys";
}
#[cfg(feature = "Foundation")]
impl ICoreAcceleratorKeysVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAcceleratorKeysImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreAcceleratorKeysVtbl {
        unsafe extern "system" fn AcceleratorKeyActivated<Impl: ICoreAcceleratorKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceleratorKeyActivated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAcceleratorKeyActivated<Impl: ICoreAcceleratorKeysImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAcceleratorKeyActivated(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAcceleratorKeys, BASE_OFFSET>(),
            AcceleratorKeyActivated: AcceleratorKeyActivated::<Impl, IMPL_OFFSET>,
            RemoveAcceleratorKeyActivated: RemoveAcceleratorKeyActivated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAcceleratorKeys as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreClosestInteractiveBoundsRequestedImpl: Sized {
    fn ClosestInteractiveBoundsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreComponentInputSource, ClosestInteractiveBoundsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosestInteractiveBoundsRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreClosestInteractiveBoundsRequested {
    const NAME: &'static str = "Windows.UI.Core.ICoreClosestInteractiveBoundsRequested";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreClosestInteractiveBoundsRequestedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreClosestInteractiveBoundsRequestedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreClosestInteractiveBoundsRequestedVtbl {
        unsafe extern "system" fn ClosestInteractiveBoundsRequested<Impl: ICoreClosestInteractiveBoundsRequestedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosestInteractiveBoundsRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreComponentInputSource, ClosestInteractiveBoundsRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreComponentInputSource, ClosestInteractiveBoundsRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosestInteractiveBoundsRequested<Impl: ICoreClosestInteractiveBoundsRequestedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosestInteractiveBoundsRequested(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreClosestInteractiveBoundsRequested, BASE_OFFSET>(),
            ClosestInteractiveBoundsRequested: ClosestInteractiveBoundsRequested::<Impl, IMPL_OFFSET>,
            RemoveClosestInteractiveBoundsRequested: RemoveClosestInteractiveBoundsRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreClosestInteractiveBoundsRequested as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreComponentFocusableImpl: Sized {
    fn HasFocus(&self) -> ::windows::core::Result<bool>;
    fn GotFocus(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CoreWindowEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CoreWindowEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreComponentFocusable {
    const NAME: &'static str = "Windows.UI.Core.ICoreComponentFocusable";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreComponentFocusableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreComponentFocusableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreComponentFocusableVtbl {
        unsafe extern "system" fn HasFocus<Impl: ICoreComponentFocusableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GotFocus<Impl: ICoreComponentFocusableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GotFocus(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CoreWindowEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CoreWindowEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGotFocus<Impl: ICoreComponentFocusableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGotFocus(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LostFocus<Impl: ICoreComponentFocusableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LostFocus(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CoreWindowEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CoreWindowEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLostFocus<Impl: ICoreComponentFocusableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLostFocus(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreComponentFocusable, BASE_OFFSET>(),
            HasFocus: HasFocus::<Impl, IMPL_OFFSET>,
            GotFocus: GotFocus::<Impl, IMPL_OFFSET>,
            RemoveGotFocus: RemoveGotFocus::<Impl, IMPL_OFFSET>,
            LostFocus: LostFocus::<Impl, IMPL_OFFSET>,
            RemoveLostFocus: RemoveLostFocus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreComponentFocusable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreCursorImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Type(&self) -> ::windows::core::Result<CoreCursorType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreCursor {
    const NAME: &'static str = "Windows.UI.Core.ICoreCursor";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreCursorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreCursorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreCursorVtbl {
        unsafe extern "system" fn Id<Impl: ICoreCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: ICoreCursorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreCursorType) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreCursor, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreCursor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreCursorFactoryImpl: Sized {
    fn CreateCursor(&self, r#type: CoreCursorType, id: u32) -> ::windows::core::Result<CoreCursor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreCursorFactory {
    const NAME: &'static str = "Windows.UI.Core.ICoreCursorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreCursorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreCursorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreCursorFactoryVtbl {
        unsafe extern "system" fn CreateCursor<Impl: ICoreCursorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: CoreCursorType, id: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCursor(r#type, id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreCursorFactory, BASE_OFFSET>(), CreateCursor: CreateCursor::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreCursorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreDispatcherImpl: Sized + ICoreAcceleratorKeysImpl {
    fn HasThreadAccess(&self) -> ::windows::core::Result<bool>;
    fn ProcessEvents(&self, options: CoreProcessEventsOption) -> ::windows::core::Result<()>;
    fn RunAsync(&self, priority: CoreDispatcherPriority, agilecallback: &::core::option::Option<DispatchedHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RunIdleAsync(&self, agilecallback: &::core::option::Option<IdleDispatchedHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreDispatcher {
    const NAME: &'static str = "Windows.UI.Core.ICoreDispatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreDispatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDispatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreDispatcherVtbl {
        unsafe extern "system" fn HasThreadAccess<Impl: ICoreDispatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasThreadAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessEvents<Impl: ICoreDispatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: CoreProcessEventsOption) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessEvents(options).into()
        }
        unsafe extern "system" fn RunAsync<Impl: ICoreDispatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: CoreDispatcherPriority, agilecallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunAsync(priority, &*(&agilecallback as *const <DispatchedHandler as ::windows::core::Abi>::Abi as *const <DispatchedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunIdleAsync<Impl: ICoreDispatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, agilecallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunIdleAsync(&*(&agilecallback as *const <IdleDispatchedHandler as ::windows::core::Abi>::Abi as *const <IdleDispatchedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreDispatcher, BASE_OFFSET>(),
            HasThreadAccess: HasThreadAccess::<Impl, IMPL_OFFSET>,
            ProcessEvents: ProcessEvents::<Impl, IMPL_OFFSET>,
            RunAsync: RunAsync::<Impl, IMPL_OFFSET>,
            RunIdleAsync: RunIdleAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreDispatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreDispatcher2Impl: Sized {
    fn TryRunAsync(&self, priority: CoreDispatcherPriority, agilecallback: &::core::option::Option<DispatchedHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryRunIdleAsync(&self, agilecallback: &::core::option::Option<IdleDispatchedHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreDispatcher2 {
    const NAME: &'static str = "Windows.UI.Core.ICoreDispatcher2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreDispatcher2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDispatcher2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreDispatcher2Vtbl {
        unsafe extern "system" fn TryRunAsync<Impl: ICoreDispatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: CoreDispatcherPriority, agilecallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRunAsync(priority, &*(&agilecallback as *const <DispatchedHandler as ::windows::core::Abi>::Abi as *const <DispatchedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryRunIdleAsync<Impl: ICoreDispatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, agilecallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryRunIdleAsync(&*(&agilecallback as *const <IdleDispatchedHandler as ::windows::core::Abi>::Abi as *const <IdleDispatchedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreDispatcher2, BASE_OFFSET>(),
            TryRunAsync: TryRunAsync::<Impl, IMPL_OFFSET>,
            TryRunIdleAsync: TryRunIdleAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreDispatcher2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreDispatcherWithTaskPriorityImpl: Sized {
    fn CurrentPriority(&self) -> ::windows::core::Result<CoreDispatcherPriority>;
    fn SetCurrentPriority(&self, value: CoreDispatcherPriority) -> ::windows::core::Result<()>;
    fn ShouldYield(&self) -> ::windows::core::Result<bool>;
    fn ShouldYieldToPriority(&self, priority: CoreDispatcherPriority) -> ::windows::core::Result<bool>;
    fn StopProcessEvents(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreDispatcherWithTaskPriority {
    const NAME: &'static str = "Windows.UI.Core.ICoreDispatcherWithTaskPriority";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreDispatcherWithTaskPriorityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreDispatcherWithTaskPriorityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreDispatcherWithTaskPriorityVtbl {
        unsafe extern "system" fn CurrentPriority<Impl: ICoreDispatcherWithTaskPriorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreDispatcherPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPriority<Impl: ICoreDispatcherWithTaskPriorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CoreDispatcherPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrentPriority(value).into()
        }
        unsafe extern "system" fn ShouldYield<Impl: ICoreDispatcherWithTaskPriorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldYield() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShouldYieldToPriority<Impl: ICoreDispatcherWithTaskPriorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: CoreDispatcherPriority, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldYieldToPriority(priority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopProcessEvents<Impl: ICoreDispatcherWithTaskPriorityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopProcessEvents().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreDispatcherWithTaskPriority, BASE_OFFSET>(),
            CurrentPriority: CurrentPriority::<Impl, IMPL_OFFSET>,
            SetCurrentPriority: SetCurrentPriority::<Impl, IMPL_OFFSET>,
            ShouldYield: ShouldYield::<Impl, IMPL_OFFSET>,
            ShouldYieldToPriority: ShouldYieldToPriority::<Impl, IMPL_OFFSET>,
            StopProcessEvents: StopProcessEvents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreDispatcherWithTaskPriority as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreIndependentInputSourceControllerImpl: Sized {
    fn IsTransparentForUncontrolledInput(&self) -> ::windows::core::Result<bool>;
    fn SetIsTransparentForUncontrolledInput(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPalmRejectionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPalmRejectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<CoreIndependentInputSource>;
    fn SetControlledInput(&self, inputtypes: CoreInputDeviceTypes) -> ::windows::core::Result<()>;
    fn SetControlledInputWithFilters(&self, inputtypes: CoreInputDeviceTypes, required: CoreIndependentInputFilters, excluded: CoreIndependentInputFilters) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreIndependentInputSourceController {
    const NAME: &'static str = "Windows.UI.Core.ICoreIndependentInputSourceController";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreIndependentInputSourceControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreIndependentInputSourceControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreIndependentInputSourceControllerVtbl {
        unsafe extern "system" fn IsTransparentForUncontrolledInput<Impl: ICoreIndependentInputSourceControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTransparentForUncontrolledInput() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTransparentForUncontrolledInput<Impl: ICoreIndependentInputSourceControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTransparentForUncontrolledInput(value).into()
        }
        unsafe extern "system" fn IsPalmRejectionEnabled<Impl: ICoreIndependentInputSourceControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPalmRejectionEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsPalmRejectionEnabled<Impl: ICoreIndependentInputSourceControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsPalmRejectionEnabled(value).into()
        }
        unsafe extern "system" fn Source<Impl: ICoreIndependentInputSourceControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetControlledInput<Impl: ICoreIndependentInputSourceControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtypes: CoreInputDeviceTypes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlledInput(inputtypes).into()
        }
        unsafe extern "system" fn SetControlledInputWithFilters<Impl: ICoreIndependentInputSourceControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputtypes: CoreInputDeviceTypes, required: CoreIndependentInputFilters, excluded: CoreIndependentInputFilters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlledInputWithFilters(inputtypes, required, excluded).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreIndependentInputSourceController, BASE_OFFSET>(),
            IsTransparentForUncontrolledInput: IsTransparentForUncontrolledInput::<Impl, IMPL_OFFSET>,
            SetIsTransparentForUncontrolledInput: SetIsTransparentForUncontrolledInput::<Impl, IMPL_OFFSET>,
            IsPalmRejectionEnabled: IsPalmRejectionEnabled::<Impl, IMPL_OFFSET>,
            SetIsPalmRejectionEnabled: SetIsPalmRejectionEnabled::<Impl, IMPL_OFFSET>,
            Source: Source::<Impl, IMPL_OFFSET>,
            SetControlledInput: SetControlledInput::<Impl, IMPL_OFFSET>,
            SetControlledInputWithFilters: SetControlledInputWithFilters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreIndependentInputSourceController as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait ICoreIndependentInputSourceControllerStaticsImpl: Sized {
    fn CreateForVisual(&self, visual: &::core::option::Option<super::Composition::Visual>) -> ::windows::core::Result<CoreIndependentInputSourceController>;
    fn CreateForIVisualElement(&self, visualelement: &::core::option::Option<super::Composition::IVisualElement>) -> ::windows::core::Result<CoreIndependentInputSourceController>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreIndependentInputSourceControllerStatics {
    const NAME: &'static str = "Windows.UI.Core.ICoreIndependentInputSourceControllerStatics";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ICoreIndependentInputSourceControllerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreIndependentInputSourceControllerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreIndependentInputSourceControllerStaticsVtbl {
        unsafe extern "system" fn CreateForVisual<Impl: ICoreIndependentInputSourceControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForVisual(&*(&visual as *const <super::Composition::Visual as ::windows::core::Abi>::Abi as *const <super::Composition::Visual as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateForIVisualElement<Impl: ICoreIndependentInputSourceControllerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visualelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForIVisualElement(&*(&visualelement as *const <super::Composition::IVisualElement as ::windows::core::Abi>::Abi as *const <super::Composition::IVisualElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreIndependentInputSourceControllerStatics, BASE_OFFSET>(),
            CreateForVisual: CreateForVisual::<Impl, IMPL_OFFSET>,
            CreateForIVisualElement: CreateForIVisualElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreIndependentInputSourceControllerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ICoreInputSourceBaseImpl: Sized {
    fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher>;
    fn IsInputEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputEnabled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, InputEnabledEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputEnabled(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICoreInputSourceBase {
    const NAME: &'static str = "Windows.UI.Core.ICoreInputSourceBase";
}
#[cfg(feature = "Foundation")]
impl ICoreInputSourceBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreInputSourceBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreInputSourceBaseVtbl {
        unsafe extern "system" fn Dispatcher<Impl: ICoreInputSourceBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInputEnabled<Impl: ICoreInputSourceBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInputEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInputEnabled<Impl: ICoreInputSourceBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInputEnabled(value).into()
        }
        unsafe extern "system" fn InputEnabled<Impl: ICoreInputSourceBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputEnabled(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, InputEnabledEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, InputEnabledEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputEnabled<Impl: ICoreInputSourceBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInputEnabled(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputSourceBase, BASE_OFFSET>(),
            Dispatcher: Dispatcher::<Impl, IMPL_OFFSET>,
            IsInputEnabled: IsInputEnabled::<Impl, IMPL_OFFSET>,
            SetIsInputEnabled: SetIsInputEnabled::<Impl, IMPL_OFFSET>,
            InputEnabled: InputEnabled::<Impl, IMPL_OFFSET>,
            RemoveInputEnabled: RemoveInputEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputSourceBase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait ICoreKeyboardInputSourceImpl: Sized {
    fn GetCurrentKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates>;
    fn CharacterReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CharacterReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCharacterReceived(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyDown(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyDown(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyUp(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyUp(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreKeyboardInputSource {
    const NAME: &'static str = "Windows.UI.Core.ICoreKeyboardInputSource";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ICoreKeyboardInputSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreKeyboardInputSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreKeyboardInputSourceVtbl {
        unsafe extern "system" fn GetCurrentKeyState<Impl: ICoreKeyboardInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentKeyState(virtualkey) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacterReceived<Impl: ICoreKeyboardInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CharacterReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, CharacterReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCharacterReceived<Impl: ICoreKeyboardInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCharacterReceived(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyDown<Impl: ICoreKeyboardInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyDown(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, KeyEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, KeyEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyDown<Impl: ICoreKeyboardInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveKeyDown(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyUp<Impl: ICoreKeyboardInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyUp(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, KeyEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, KeyEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyUp<Impl: ICoreKeyboardInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveKeyUp(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreKeyboardInputSource, BASE_OFFSET>(),
            GetCurrentKeyState: GetCurrentKeyState::<Impl, IMPL_OFFSET>,
            CharacterReceived: CharacterReceived::<Impl, IMPL_OFFSET>,
            RemoveCharacterReceived: RemoveCharacterReceived::<Impl, IMPL_OFFSET>,
            KeyDown: KeyDown::<Impl, IMPL_OFFSET>,
            RemoveKeyDown: RemoveKeyDown::<Impl, IMPL_OFFSET>,
            KeyUp: KeyUp::<Impl, IMPL_OFFSET>,
            RemoveKeyUp: RemoveKeyUp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreKeyboardInputSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreKeyboardInputSource2Impl: Sized {
    fn GetCurrentKeyEventDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreKeyboardInputSource2 {
    const NAME: &'static str = "Windows.UI.Core.ICoreKeyboardInputSource2";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreKeyboardInputSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreKeyboardInputSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreKeyboardInputSource2Vtbl {
        unsafe extern "system" fn GetCurrentKeyEventDeviceId<Impl: ICoreKeyboardInputSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentKeyEventDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreKeyboardInputSource2, BASE_OFFSET>(),
            GetCurrentKeyEventDeviceId: GetCurrentKeyEventDeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreKeyboardInputSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ICorePointerInputSourceImpl: Sized {
    fn ReleasePointerCapture(&self) -> ::windows::core::Result<()>;
    fn SetPointerCapture(&self) -> ::windows::core::Result<()>;
    fn HasCapture(&self) -> ::windows::core::Result<bool>;
    fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor>;
    fn SetPointerCursor(&self, value: &::core::option::Option<CoreCursor>) -> ::windows::core::Result<()>;
    fn PointerCaptureLost(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCaptureLost(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerEntered(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExited(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleased(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerWheelChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerWheelChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICorePointerInputSource {
    const NAME: &'static str = "Windows.UI.Core.ICorePointerInputSource";
}
#[cfg(feature = "Foundation")]
impl ICorePointerInputSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICorePointerInputSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICorePointerInputSourceVtbl {
        unsafe extern "system" fn ReleasePointerCapture<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleasePointerCapture().into()
        }
        unsafe extern "system" fn SetPointerCapture<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerCapture().into()
        }
        unsafe extern "system" fn HasCapture<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCapture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerPosition<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerCursor<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerCursor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerCursor<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerCursor(&*(&value as *const <CoreCursor as ::windows::core::Abi>::Abi as *const <CoreCursor as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerCaptureLost<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerCaptureLost(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerCaptureLost(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerEntered<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerEntered(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerEntered(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerExited<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerExited(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerExited<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerExited(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerMoved<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerMoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerMoved(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerPressed<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerPressed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerPressed(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerReleased<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerReleased(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerReleased(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerWheelChanged<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerWheelChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Impl: ICorePointerInputSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerWheelChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICorePointerInputSource, BASE_OFFSET>(),
            ReleasePointerCapture: ReleasePointerCapture::<Impl, IMPL_OFFSET>,
            SetPointerCapture: SetPointerCapture::<Impl, IMPL_OFFSET>,
            HasCapture: HasCapture::<Impl, IMPL_OFFSET>,
            PointerPosition: PointerPosition::<Impl, IMPL_OFFSET>,
            PointerCursor: PointerCursor::<Impl, IMPL_OFFSET>,
            SetPointerCursor: SetPointerCursor::<Impl, IMPL_OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Impl, IMPL_OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Impl, IMPL_OFFSET>,
            PointerEntered: PointerEntered::<Impl, IMPL_OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Impl, IMPL_OFFSET>,
            PointerExited: PointerExited::<Impl, IMPL_OFFSET>,
            RemovePointerExited: RemovePointerExited::<Impl, IMPL_OFFSET>,
            PointerMoved: PointerMoved::<Impl, IMPL_OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Impl, IMPL_OFFSET>,
            PointerPressed: PointerPressed::<Impl, IMPL_OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Impl, IMPL_OFFSET>,
            PointerReleased: PointerReleased::<Impl, IMPL_OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Impl, IMPL_OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Impl, IMPL_OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorePointerInputSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System"))]
pub trait ICorePointerInputSource2Impl: Sized + ICorePointerInputSourceImpl {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ::windows::core::RuntimeName for ICorePointerInputSource2 {
    const NAME: &'static str = "Windows.UI.Core.ICorePointerInputSource2";
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ICorePointerInputSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICorePointerInputSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICorePointerInputSource2Vtbl {
        unsafe extern "system" fn DispatcherQueue<Impl: ICorePointerInputSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICorePointerInputSource2, BASE_OFFSET>(),
            DispatcherQueue: DispatcherQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorePointerInputSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ICorePointerRedirectorImpl: Sized {
    fn PointerRoutedAway(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerRoutedAway(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerRoutedTo(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerRoutedTo(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerRoutedReleased(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerRoutedReleased(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICorePointerRedirector {
    const NAME: &'static str = "Windows.UI.Core.ICorePointerRedirector";
}
#[cfg(feature = "Foundation")]
impl ICorePointerRedirectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICorePointerRedirectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICorePointerRedirectorVtbl {
        unsafe extern "system" fn PointerRoutedAway<Impl: ICorePointerRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerRoutedAway(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerRoutedAway<Impl: ICorePointerRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerRoutedAway(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerRoutedTo<Impl: ICorePointerRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerRoutedTo(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerRoutedTo<Impl: ICorePointerRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerRoutedTo(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerRoutedReleased<Impl: ICorePointerRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerRoutedReleased(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerRoutedReleased<Impl: ICorePointerRedirectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerRoutedReleased(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICorePointerRedirector, BASE_OFFSET>(),
            PointerRoutedAway: PointerRoutedAway::<Impl, IMPL_OFFSET>,
            RemovePointerRoutedAway: RemovePointerRoutedAway::<Impl, IMPL_OFFSET>,
            PointerRoutedTo: PointerRoutedTo::<Impl, IMPL_OFFSET>,
            RemovePointerRoutedTo: RemovePointerRoutedTo::<Impl, IMPL_OFFSET>,
            PointerRoutedReleased: PointerRoutedReleased::<Impl, IMPL_OFFSET>,
            RemovePointerRoutedReleased: RemovePointerRoutedReleased::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorePointerRedirector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreTouchHitTestingImpl: Sized {
    fn TouchHitTesting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TouchHitTestingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTouchHitTesting(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreTouchHitTesting {
    const NAME: &'static str = "Windows.UI.Core.ICoreTouchHitTesting";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreTouchHitTestingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreTouchHitTestingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreTouchHitTestingVtbl {
        unsafe extern "system" fn TouchHitTesting<Impl: ICoreTouchHitTestingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TouchHitTesting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TouchHitTestingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, TouchHitTestingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTouchHitTesting<Impl: ICoreTouchHitTestingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTouchHitTesting(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreTouchHitTesting, BASE_OFFSET>(),
            TouchHitTesting: TouchHitTesting::<Impl, IMPL_OFFSET>,
            RemoveTouchHitTesting: RemoveTouchHitTesting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreTouchHitTesting as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))]
pub trait ICoreWindowImpl: Sized {
    fn AutomationHostProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn CustomProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn Dispatcher(&self) -> ::windows::core::Result<CoreDispatcher>;
    fn FlowDirection(&self) -> ::windows::core::Result<CoreWindowFlowDirection>;
    fn SetFlowDirection(&self, value: CoreWindowFlowDirection) -> ::windows::core::Result<()>;
    fn IsInputEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PointerCursor(&self) -> ::windows::core::Result<CoreCursor>;
    fn SetPointerCursor(&self, value: &::core::option::Option<CoreCursor>) -> ::windows::core::Result<()>;
    fn PointerPosition(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn Activate(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn GetAsyncKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates>;
    fn GetKeyState(&self, virtualkey: super::super::System::VirtualKey) -> ::windows::core::Result<CoreVirtualKeyStates>;
    fn ReleasePointerCapture(&self) -> ::windows::core::Result<()>;
    fn SetPointerCapture(&self) -> ::windows::core::Result<()>;
    fn Activated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, WindowActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutomationProviderRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, AutomationProviderRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutomationProviderRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CharacterReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, CharacterReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCharacterReceived(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn InputEnabled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, InputEnabledEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputEnabled(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyDown(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyDown(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyUp(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyUp(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerCaptureLost(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCaptureLost(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerEntered(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExited(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleased(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TouchHitTesting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, TouchHitTestingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTouchHitTesting(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerWheelChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerWheelChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SizeChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VisibilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, VisibilityChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibilityChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))]
impl ::windows::core::RuntimeName for ICoreWindow {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindow";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System"))]
impl ICoreWindowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindowVtbl {
        unsafe extern "system" fn AutomationHostProvider<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomationHostProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bounds<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CustomProperties<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatcher<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FlowDirection<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreWindowFlowDirection) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFlowDirection<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CoreWindowFlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlowDirection(value).into()
        }
        unsafe extern "system" fn IsInputEnabled<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInputEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInputEnabled<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInputEnabled(value).into()
        }
        unsafe extern "system" fn PointerCursor<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerCursor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerCursor<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerCursor(&*(&value as *const <CoreCursor as ::windows::core::Abi>::Abi as *const <CoreCursor as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerPosition<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Visible<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Activate<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate().into()
        }
        unsafe extern "system" fn Close<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn GetAsyncKeyState<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsyncKeyState(virtualkey) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyState<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeyState(virtualkey) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleasePointerCapture<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleasePointerCapture().into()
        }
        unsafe extern "system" fn SetPointerCapture<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerCapture().into()
        }
        unsafe extern "system" fn Activated<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, WindowActivatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, WindowActivatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActivated(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutomationProviderRequested<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomationProviderRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, AutomationProviderRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, AutomationProviderRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAutomationProviderRequested<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAutomationProviderRequested(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CharacterReceived<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, CharacterReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, CharacterReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCharacterReceived<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCharacterReceived(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn InputEnabled<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputEnabled(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, InputEnabledEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, InputEnabledEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputEnabled<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInputEnabled(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyDown<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyDown(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyDown<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveKeyDown(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyUp<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyUp(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyUp<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveKeyUp(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerCaptureLost<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerCaptureLost(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerCaptureLost(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerEntered<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerEntered(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerEntered(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerExited<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerExited(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerExited<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerExited(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerMoved<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerMoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerMoved(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerPressed<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerPressed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerPressed(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerReleased<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerReleased(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerReleased(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TouchHitTesting<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TouchHitTesting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, TouchHitTestingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, TouchHitTestingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTouchHitTesting<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTouchHitTesting(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerWheelChanged<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerWheelChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerWheelChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SizeChanged<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSizeChanged<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSizeChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VisibilityChanged<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisibilityChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, VisibilityChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, VisibilityChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisibilityChanged<Impl: ICoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVisibilityChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindow, BASE_OFFSET>(),
            AutomationHostProvider: AutomationHostProvider::<Impl, IMPL_OFFSET>,
            Bounds: Bounds::<Impl, IMPL_OFFSET>,
            CustomProperties: CustomProperties::<Impl, IMPL_OFFSET>,
            Dispatcher: Dispatcher::<Impl, IMPL_OFFSET>,
            FlowDirection: FlowDirection::<Impl, IMPL_OFFSET>,
            SetFlowDirection: SetFlowDirection::<Impl, IMPL_OFFSET>,
            IsInputEnabled: IsInputEnabled::<Impl, IMPL_OFFSET>,
            SetIsInputEnabled: SetIsInputEnabled::<Impl, IMPL_OFFSET>,
            PointerCursor: PointerCursor::<Impl, IMPL_OFFSET>,
            SetPointerCursor: SetPointerCursor::<Impl, IMPL_OFFSET>,
            PointerPosition: PointerPosition::<Impl, IMPL_OFFSET>,
            Visible: Visible::<Impl, IMPL_OFFSET>,
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            GetAsyncKeyState: GetAsyncKeyState::<Impl, IMPL_OFFSET>,
            GetKeyState: GetKeyState::<Impl, IMPL_OFFSET>,
            ReleasePointerCapture: ReleasePointerCapture::<Impl, IMPL_OFFSET>,
            SetPointerCapture: SetPointerCapture::<Impl, IMPL_OFFSET>,
            Activated: Activated::<Impl, IMPL_OFFSET>,
            RemoveActivated: RemoveActivated::<Impl, IMPL_OFFSET>,
            AutomationProviderRequested: AutomationProviderRequested::<Impl, IMPL_OFFSET>,
            RemoveAutomationProviderRequested: RemoveAutomationProviderRequested::<Impl, IMPL_OFFSET>,
            CharacterReceived: CharacterReceived::<Impl, IMPL_OFFSET>,
            RemoveCharacterReceived: RemoveCharacterReceived::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
            InputEnabled: InputEnabled::<Impl, IMPL_OFFSET>,
            RemoveInputEnabled: RemoveInputEnabled::<Impl, IMPL_OFFSET>,
            KeyDown: KeyDown::<Impl, IMPL_OFFSET>,
            RemoveKeyDown: RemoveKeyDown::<Impl, IMPL_OFFSET>,
            KeyUp: KeyUp::<Impl, IMPL_OFFSET>,
            RemoveKeyUp: RemoveKeyUp::<Impl, IMPL_OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Impl, IMPL_OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Impl, IMPL_OFFSET>,
            PointerEntered: PointerEntered::<Impl, IMPL_OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Impl, IMPL_OFFSET>,
            PointerExited: PointerExited::<Impl, IMPL_OFFSET>,
            RemovePointerExited: RemovePointerExited::<Impl, IMPL_OFFSET>,
            PointerMoved: PointerMoved::<Impl, IMPL_OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Impl, IMPL_OFFSET>,
            PointerPressed: PointerPressed::<Impl, IMPL_OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Impl, IMPL_OFFSET>,
            PointerReleased: PointerReleased::<Impl, IMPL_OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Impl, IMPL_OFFSET>,
            TouchHitTesting: TouchHitTesting::<Impl, IMPL_OFFSET>,
            RemoveTouchHitTesting: RemoveTouchHitTesting::<Impl, IMPL_OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Impl, IMPL_OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Impl, IMPL_OFFSET>,
            SizeChanged: SizeChanged::<Impl, IMPL_OFFSET>,
            RemoveSizeChanged: RemoveSizeChanged::<Impl, IMPL_OFFSET>,
            VisibilityChanged: VisibilityChanged::<Impl, IMPL_OFFSET>,
            RemoveVisibilityChanged: RemoveVisibilityChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreWindow2Impl: Sized {
    fn SetPointerPosition(&self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreWindow2 {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindow2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreWindow2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindow2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindow2Vtbl {
        unsafe extern "system" fn SetPointerPosition<Impl: ICoreWindow2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerPosition(&*(&value as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindow2, BASE_OFFSET>(),
            SetPointerPosition: SetPointerPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindow2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreWindow3Impl: Sized {
    fn ClosestInteractiveBoundsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, ClosestInteractiveBoundsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosestInteractiveBoundsRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetCurrentKeyEventDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreWindow3 {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindow3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreWindow3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindow3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindow3Vtbl {
        unsafe extern "system" fn ClosestInteractiveBoundsRequested<Impl: ICoreWindow3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClosestInteractiveBoundsRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, ClosestInteractiveBoundsRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, ClosestInteractiveBoundsRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosestInteractiveBoundsRequested<Impl: ICoreWindow3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosestInteractiveBoundsRequested(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetCurrentKeyEventDeviceId<Impl: ICoreWindow3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentKeyEventDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindow3, BASE_OFFSET>(),
            ClosestInteractiveBoundsRequested: ClosestInteractiveBoundsRequested::<Impl, IMPL_OFFSET>,
            RemoveClosestInteractiveBoundsRequested: RemoveClosestInteractiveBoundsRequested::<Impl, IMPL_OFFSET>,
            GetCurrentKeyEventDeviceId: GetCurrentKeyEventDeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindow3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreWindow4Impl: Sized {
    fn ResizeStarted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResizeStarted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResizeCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResizeCompleted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreWindow4 {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindow4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreWindow4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindow4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindow4Vtbl {
        unsafe extern "system" fn ResizeStarted<Impl: ICoreWindow4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResizeStarted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResizeStarted<Impl: ICoreWindow4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveResizeStarted(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResizeCompleted<Impl: ICoreWindow4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResizeCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResizeCompleted<Impl: ICoreWindow4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveResizeCompleted(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindow4, BASE_OFFSET>(),
            ResizeStarted: ResizeStarted::<Impl, IMPL_OFFSET>,
            RemoveResizeStarted: RemoveResizeStarted::<Impl, IMPL_OFFSET>,
            ResizeCompleted: ResizeCompleted::<Impl, IMPL_OFFSET>,
            RemoveResizeCompleted: RemoveResizeCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindow4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait ICoreWindow5Impl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
    fn ActivationMode(&self) -> ::windows::core::Result<CoreWindowActivationMode>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreWindow5 {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindow5";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ICoreWindow5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindow5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindow5Vtbl {
        unsafe extern "system" fn DispatcherQueue<Impl: ICoreWindow5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActivationMode<Impl: ICoreWindow5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreWindowActivationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindow5, BASE_OFFSET>(),
            DispatcherQueue: DispatcherQueue::<Impl, IMPL_OFFSET>,
            ActivationMode: ActivationMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindow5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait ICoreWindowDialogImpl: Sized {
    fn Showing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShowing(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MaxSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MinSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsInteractionDelayed(&self) -> ::windows::core::Result<i32>;
    fn SetIsInteractionDelayed(&self, value: i32) -> ::windows::core::Result<()>;
    fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>>;
    fn DefaultCommandIndex(&self) -> ::windows::core::Result<u32>;
    fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::core::Result<()>;
    fn CancelCommandIndex(&self) -> ::windows::core::Result<u32>;
    fn SetCancelCommandIndex(&self, value: u32) -> ::windows::core::Result<()>;
    fn BackButtonCommand(&self) -> ::windows::core::Result<super::Popups::UICommandInvokedHandler>;
    fn SetBackButtonCommand(&self, value: &::core::option::Option<super::Popups::UICommandInvokedHandler>) -> ::windows::core::Result<()>;
    fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreWindowDialog {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowDialog";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ICoreWindowDialogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindowDialogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindowDialogVtbl {
        unsafe extern "system" fn Showing<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Showing(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShowing<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShowing(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxSize<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinSize<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsInteractionDelayed<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInteractionDelayed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInteractionDelayed<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInteractionDelayed(value).into()
        }
        unsafe extern "system" fn Commands<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultCommandIndex<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultCommandIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultCommandIndex<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultCommandIndex(value).into()
        }
        unsafe extern "system" fn CancelCommandIndex<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelCommandIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCancelCommandIndex<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancelCommandIndex(value).into()
        }
        unsafe extern "system" fn BackButtonCommand<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackButtonCommand() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackButtonCommand<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackButtonCommand(&*(&value as *const <super::Popups::UICommandInvokedHandler as ::windows::core::Abi>::Abi as *const <super::Popups::UICommandInvokedHandler as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowAsync<Impl: ICoreWindowDialogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindowDialog, BASE_OFFSET>(),
            Showing: Showing::<Impl, IMPL_OFFSET>,
            RemoveShowing: RemoveShowing::<Impl, IMPL_OFFSET>,
            MaxSize: MaxSize::<Impl, IMPL_OFFSET>,
            MinSize: MinSize::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            IsInteractionDelayed: IsInteractionDelayed::<Impl, IMPL_OFFSET>,
            SetIsInteractionDelayed: SetIsInteractionDelayed::<Impl, IMPL_OFFSET>,
            Commands: Commands::<Impl, IMPL_OFFSET>,
            DefaultCommandIndex: DefaultCommandIndex::<Impl, IMPL_OFFSET>,
            SetDefaultCommandIndex: SetDefaultCommandIndex::<Impl, IMPL_OFFSET>,
            CancelCommandIndex: CancelCommandIndex::<Impl, IMPL_OFFSET>,
            SetCancelCommandIndex: SetCancelCommandIndex::<Impl, IMPL_OFFSET>,
            BackButtonCommand: BackButtonCommand::<Impl, IMPL_OFFSET>,
            SetBackButtonCommand: SetBackButtonCommand::<Impl, IMPL_OFFSET>,
            ShowAsync: ShowAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowDialog as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowDialogFactoryImpl: Sized {
    fn CreateWithTitle(&self, title: &::windows::core::HSTRING) -> ::windows::core::Result<CoreWindowDialog>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreWindowDialogFactory {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowDialogFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreWindowDialogFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindowDialogFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindowDialogFactoryVtbl {
        unsafe extern "system" fn CreateWithTitle<Impl: ICoreWindowDialogFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithTitle(&*(&title as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindowDialogFactory, BASE_OFFSET>(),
            CreateWithTitle: CreateWithTitle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowDialogFactory as ::windows::core::Interface>::IID
    }
}
pub trait ICoreWindowEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICoreWindowEventArgs {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowEventArgs";
}
impl ICoreWindowEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindowEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindowEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: ICoreWindowEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: ICoreWindowEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindowEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait ICoreWindowFlyoutImpl: Sized {
    fn Showing(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShowing(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MaxSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn MinSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsInteractionDelayed(&self) -> ::windows::core::Result<i32>;
    fn SetIsInteractionDelayed(&self, value: i32) -> ::windows::core::Result<()>;
    fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Popups::IUICommand>>;
    fn DefaultCommandIndex(&self) -> ::windows::core::Result<u32>;
    fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::core::Result<()>;
    fn BackButtonCommand(&self) -> ::windows::core::Result<super::Popups::UICommandInvokedHandler>;
    fn SetBackButtonCommand(&self, value: &::core::option::Option<super::Popups::UICommandInvokedHandler>) -> ::windows::core::Result<()>;
    fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Popups::IUICommand>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreWindowFlyout {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowFlyout";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Popups", feature = "implement_exclusive"))]
impl ICoreWindowFlyoutVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindowFlyoutImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindowFlyoutVtbl {
        unsafe extern "system" fn Showing<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Showing(&*(&handler as *const <super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowPopupShowingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveShowing<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveShowing(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MaxSize<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinSize<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsInteractionDelayed<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInteractionDelayed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInteractionDelayed<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInteractionDelayed(value).into()
        }
        unsafe extern "system" fn Commands<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultCommandIndex<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultCommandIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultCommandIndex<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultCommandIndex(value).into()
        }
        unsafe extern "system" fn BackButtonCommand<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackButtonCommand() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackButtonCommand<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackButtonCommand(&*(&value as *const <super::Popups::UICommandInvokedHandler as ::windows::core::Abi>::Abi as *const <super::Popups::UICommandInvokedHandler as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowAsync<Impl: ICoreWindowFlyoutImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindowFlyout, BASE_OFFSET>(),
            Showing: Showing::<Impl, IMPL_OFFSET>,
            RemoveShowing: RemoveShowing::<Impl, IMPL_OFFSET>,
            MaxSize: MaxSize::<Impl, IMPL_OFFSET>,
            MinSize: MinSize::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            IsInteractionDelayed: IsInteractionDelayed::<Impl, IMPL_OFFSET>,
            SetIsInteractionDelayed: SetIsInteractionDelayed::<Impl, IMPL_OFFSET>,
            Commands: Commands::<Impl, IMPL_OFFSET>,
            DefaultCommandIndex: DefaultCommandIndex::<Impl, IMPL_OFFSET>,
            SetDefaultCommandIndex: SetDefaultCommandIndex::<Impl, IMPL_OFFSET>,
            BackButtonCommand: BackButtonCommand::<Impl, IMPL_OFFSET>,
            SetBackButtonCommand: SetBackButtonCommand::<Impl, IMPL_OFFSET>,
            ShowAsync: ShowAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowFlyout as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreWindowFlyoutFactoryImpl: Sized {
    fn Create(&self, position: &super::super::Foundation::Point) -> ::windows::core::Result<CoreWindowFlyout>;
    fn CreateWithTitle(&self, position: &super::super::Foundation::Point, title: &::windows::core::HSTRING) -> ::windows::core::Result<CoreWindowFlyout>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreWindowFlyoutFactory {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowFlyoutFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreWindowFlyoutFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindowFlyoutFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindowFlyoutFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ICoreWindowFlyoutFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&position as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithTitle<Impl: ICoreWindowFlyoutFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: super::super::Foundation::Point, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithTitle(&*(&position as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&title as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindowFlyoutFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithTitle: CreateWithTitle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowFlyoutFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICoreWindowPopupShowingEventArgsImpl: Sized {
    fn SetDesiredSize(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICoreWindowPopupShowingEventArgs {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowPopupShowingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICoreWindowPopupShowingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindowPopupShowingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindowPopupShowingEventArgsVtbl {
        unsafe extern "system" fn SetDesiredSize<Impl: ICoreWindowPopupShowingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredSize(&*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindowPopupShowingEventArgs, BASE_OFFSET>(),
            SetDesiredSize: SetDesiredSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowPopupShowingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowResizeManagerImpl: Sized {
    fn NotifyLayoutCompleted(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreWindowResizeManager {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowResizeManager";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreWindowResizeManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindowResizeManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindowResizeManagerVtbl {
        unsafe extern "system" fn NotifyLayoutCompleted<Impl: ICoreWindowResizeManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyLayoutCompleted().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindowResizeManager, BASE_OFFSET>(),
            NotifyLayoutCompleted: NotifyLayoutCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowResizeManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowResizeManagerLayoutCapabilityImpl: Sized {
    fn SetShouldWaitForLayoutCompletion(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShouldWaitForLayoutCompletion(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreWindowResizeManagerLayoutCapability {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowResizeManagerLayoutCapability";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreWindowResizeManagerLayoutCapabilityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindowResizeManagerLayoutCapabilityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindowResizeManagerLayoutCapabilityVtbl {
        unsafe extern "system" fn SetShouldWaitForLayoutCompletion<Impl: ICoreWindowResizeManagerLayoutCapabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldWaitForLayoutCompletion(value).into()
        }
        unsafe extern "system" fn ShouldWaitForLayoutCompletion<Impl: ICoreWindowResizeManagerLayoutCapabilityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldWaitForLayoutCompletion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindowResizeManagerLayoutCapability, BASE_OFFSET>(),
            SetShouldWaitForLayoutCompletion: SetShouldWaitForLayoutCompletion::<Impl, IMPL_OFFSET>,
            ShouldWaitForLayoutCompletion: ShouldWaitForLayoutCompletion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowResizeManagerLayoutCapability as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowResizeManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<CoreWindowResizeManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreWindowResizeManagerStatics {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowResizeManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreWindowResizeManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindowResizeManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindowResizeManagerStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ICoreWindowResizeManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindowResizeManagerStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowResizeManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowStaticImpl: Sized {
    fn GetForCurrentThread(&self) -> ::windows::core::Result<CoreWindow>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreWindowStatic {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowStatic";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreWindowStaticVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindowStaticImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindowStaticVtbl {
        unsafe extern "system" fn GetForCurrentThread<Impl: ICoreWindowStaticImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentThread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindowStatic, BASE_OFFSET>(),
            GetForCurrentThread: GetForCurrentThread::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowStatic as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreWindowWithContextImpl: Sized {
    fn UIContext(&self) -> ::windows::core::Result<super::UIContext>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreWindowWithContext {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowWithContext";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreWindowWithContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreWindowWithContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreWindowWithContextVtbl {
        unsafe extern "system" fn UIContext<Impl: ICoreWindowWithContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindowWithContext, BASE_OFFSET>(), UIContext: UIContext::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowWithContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIdleDispatchedHandlerArgsImpl: Sized {
    fn IsDispatcherIdle(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIdleDispatchedHandlerArgs {
    const NAME: &'static str = "Windows.UI.Core.IIdleDispatchedHandlerArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IIdleDispatchedHandlerArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIdleDispatchedHandlerArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIdleDispatchedHandlerArgsVtbl {
        unsafe extern "system" fn IsDispatcherIdle<Impl: IIdleDispatchedHandlerArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDispatcherIdle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IIdleDispatchedHandlerArgs, BASE_OFFSET>(),
            IsDispatcherIdle: IsDispatcherIdle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIdleDispatchedHandlerArgs as ::windows::core::Interface>::IID
    }
}
pub trait IInitializeWithCoreWindowImpl: Sized {
    fn Initialize(&self, window: &::core::option::Option<CoreWindow>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInitializeWithCoreWindow {
    const NAME: &'static str = "Windows.UI.Core.IInitializeWithCoreWindow";
}
impl IInitializeWithCoreWindowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInitializeWithCoreWindowImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInitializeWithCoreWindowVtbl {
        unsafe extern "system" fn Initialize<Impl: IInitializeWithCoreWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(&*(&window as *const <CoreWindow as ::windows::core::Abi>::Abi as *const <CoreWindow as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInitializeWithCoreWindow, BASE_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInitializeWithCoreWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputEnabledEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn InputEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputEnabledEventArgs {
    const NAME: &'static str = "Windows.UI.Core.IInputEnabledEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IInputEnabledEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputEnabledEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputEnabledEventArgsVtbl {
        unsafe extern "system" fn InputEnabled<Impl: IInputEnabledEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInputEnabledEventArgs, BASE_OFFSET>(), InputEnabled: InputEnabled::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputEnabledEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IKeyEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn VirtualKey(&self) -> ::windows::core::Result<super::super::System::VirtualKey>;
    fn KeyStatus(&self) -> ::windows::core::Result<CorePhysicalKeyStatus>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyEventArgs {
    const NAME: &'static str = "Windows.UI.Core.IKeyEventArgs";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IKeyEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyEventArgsVtbl {
        unsafe extern "system" fn VirtualKey<Impl: IKeyEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VirtualKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyStatus<Impl: IKeyEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CorePhysicalKeyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyEventArgs, BASE_OFFSET>(),
            VirtualKey: VirtualKey::<Impl, IMPL_OFFSET>,
            KeyStatus: KeyStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyEventArgs2Impl: Sized + ICoreWindowEventArgsImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyEventArgs2 {
    const NAME: &'static str = "Windows.UI.Core.IKeyEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyEventArgs2Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IKeyEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyEventArgs2, BASE_OFFSET>(), DeviceId: DeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "System", feature = "UI_Input", feature = "implement_exclusive"))]
pub trait IPointerEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn CurrentPoint(&self) -> ::windows::core::Result<super::Input::PointerPoint>;
    fn KeyModifiers(&self) -> ::windows::core::Result<super::super::System::VirtualKeyModifiers>;
    fn GetIntermediatePoints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::Input::PointerPoint>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "System", feature = "UI_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointerEventArgs {
    const NAME: &'static str = "Windows.UI.Core.IPointerEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "System", feature = "UI_Input", feature = "implement_exclusive"))]
impl IPointerEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerEventArgsVtbl {
        unsafe extern "system" fn CurrentPoint<Impl: IPointerEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyModifiers<Impl: IPointerEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntermediatePoints<Impl: IPointerEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntermediatePoints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerEventArgs, BASE_OFFSET>(),
            CurrentPoint: CurrentPoint::<Impl, IMPL_OFFSET>,
            KeyModifiers: KeyModifiers::<Impl, IMPL_OFFSET>,
            GetIntermediatePoints: GetIntermediatePoints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISystemNavigationManagerImpl: Sized {
    fn BackRequested(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<BackRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISystemNavigationManager {
    const NAME: &'static str = "Windows.UI.Core.ISystemNavigationManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISystemNavigationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemNavigationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemNavigationManagerVtbl {
        unsafe extern "system" fn BackRequested<Impl: ISystemNavigationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackRequested(&*(&handler as *const <super::super::Foundation::EventHandler<BackRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<BackRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBackRequested<Impl: ISystemNavigationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBackRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemNavigationManager, BASE_OFFSET>(),
            BackRequested: BackRequested::<Impl, IMPL_OFFSET>,
            RemoveBackRequested: RemoveBackRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemNavigationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemNavigationManager2Impl: Sized {
    fn AppViewBackButtonVisibility(&self) -> ::windows::core::Result<AppViewBackButtonVisibility>;
    fn SetAppViewBackButtonVisibility(&self, value: AppViewBackButtonVisibility) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemNavigationManager2 {
    const NAME: &'static str = "Windows.UI.Core.ISystemNavigationManager2";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemNavigationManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemNavigationManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemNavigationManager2Vtbl {
        unsafe extern "system" fn AppViewBackButtonVisibility<Impl: ISystemNavigationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppViewBackButtonVisibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppViewBackButtonVisibility() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppViewBackButtonVisibility<Impl: ISystemNavigationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppViewBackButtonVisibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppViewBackButtonVisibility(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemNavigationManager2, BASE_OFFSET>(),
            AppViewBackButtonVisibility: AppViewBackButtonVisibility::<Impl, IMPL_OFFSET>,
            SetAppViewBackButtonVisibility: SetAppViewBackButtonVisibility::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemNavigationManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemNavigationManagerStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<SystemNavigationManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISystemNavigationManagerStatics {
    const NAME: &'static str = "Windows.UI.Core.ISystemNavigationManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISystemNavigationManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemNavigationManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemNavigationManagerStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ISystemNavigationManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemNavigationManagerStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemNavigationManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITouchHitTestingEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn ProximityEvaluation(&self) -> ::windows::core::Result<CoreProximityEvaluation>;
    fn SetProximityEvaluation(&self, value: &CoreProximityEvaluation) -> ::windows::core::Result<()>;
    fn Point(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn BoundingBox(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn EvaluateProximityToRect(&self, controlboundingbox: &super::super::Foundation::Rect) -> ::windows::core::Result<CoreProximityEvaluation>;
    fn EvaluateProximityToPolygon(&self, controlvertices: &[<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<CoreProximityEvaluation>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITouchHitTestingEventArgs {
    const NAME: &'static str = "Windows.UI.Core.ITouchHitTestingEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ITouchHitTestingEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITouchHitTestingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITouchHitTestingEventArgsVtbl {
        unsafe extern "system" fn ProximityEvaluation<Impl: ITouchHitTestingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreProximityEvaluation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProximityEvaluation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProximityEvaluation<Impl: ITouchHitTestingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CoreProximityEvaluation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProximityEvaluation(&*(&value as *const <CoreProximityEvaluation as ::windows::core::Abi>::Abi as *const <CoreProximityEvaluation as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Point<Impl: ITouchHitTestingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Point() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BoundingBox<Impl: ITouchHitTestingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoundingBox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluateProximityToRect<Impl: ITouchHitTestingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controlboundingbox: super::super::Foundation::Rect, result__: *mut CoreProximityEvaluation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EvaluateProximityToRect(&*(&controlboundingbox as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EvaluateProximityToPolygon<Impl: ITouchHitTestingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controlVertices_array_size: u32, controlvertices: *const super::super::Foundation::Point, result__: *mut CoreProximityEvaluation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EvaluateProximityToPolygon(::core::slice::from_raw_parts(::core::mem::transmute_copy(&controlvertices), controlVertices_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITouchHitTestingEventArgs, BASE_OFFSET>(),
            ProximityEvaluation: ProximityEvaluation::<Impl, IMPL_OFFSET>,
            SetProximityEvaluation: SetProximityEvaluation::<Impl, IMPL_OFFSET>,
            Point: Point::<Impl, IMPL_OFFSET>,
            BoundingBox: BoundingBox::<Impl, IMPL_OFFSET>,
            EvaluateProximityToRect: EvaluateProximityToRect::<Impl, IMPL_OFFSET>,
            EvaluateProximityToPolygon: EvaluateProximityToPolygon::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITouchHitTestingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisibilityChangedEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn Visible(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisibilityChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.IVisibilityChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVisibilityChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisibilityChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisibilityChangedEventArgsVtbl {
        unsafe extern "system" fn Visible<Impl: IVisibilityChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisibilityChangedEventArgs, BASE_OFFSET>(), Visible: Visible::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisibilityChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowActivatedEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn WindowActivationState(&self) -> ::windows::core::Result<CoreWindowActivationState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.IWindowActivatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowActivatedEventArgsVtbl {
        unsafe extern "system" fn WindowActivationState<Impl: IWindowActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreWindowActivationState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WindowActivationState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowActivatedEventArgs, BASE_OFFSET>(),
            WindowActivationState: WindowActivationState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWindowSizeChangedEventArgsImpl: Sized + ICoreWindowEventArgsImpl {
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWindowSizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Core.IWindowSizeChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWindowSizeChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowSizeChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowSizeChangedEventArgsVtbl {
        unsafe extern "system" fn Size<Impl: IWindowSizeChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowSizeChangedEventArgs, BASE_OFFSET>(), Size: Size::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowSizeChangedEventArgs as ::windows::core::Interface>::IID
    }
}
