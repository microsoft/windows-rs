#[cfg(feature = "Foundation")]
pub trait ICoreAcceleratorKeys_Impl: Sized {
    fn AcceleratorKeyActivated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAcceleratorKeyActivated(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICoreAcceleratorKeys {
    const NAME: &'static str = "Windows.UI.Core.ICoreAcceleratorKeys";
}
#[cfg(feature = "Foundation")]
impl ICoreAcceleratorKeys_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreAcceleratorKeys_Impl, const OFFSET: isize>() -> ICoreAcceleratorKeys_Vtbl {
        unsafe extern "system" fn AcceleratorKeyActivated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreAcceleratorKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AcceleratorKeyActivated(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAcceleratorKeyActivated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreAcceleratorKeys_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAcceleratorKeyActivated(::core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAcceleratorKeys, OFFSET>(),
            AcceleratorKeyActivated: AcceleratorKeyActivated::<Identity, Impl, OFFSET>,
            RemoveAcceleratorKeyActivated: RemoveAcceleratorKeyActivated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAcceleratorKeys as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ICoreInputSourceBase_Impl: Sized {
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
impl ICoreInputSourceBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: isize>() -> ICoreInputSourceBase_Vtbl {
        unsafe extern "system" fn Dispatcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Dispatcher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInputEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInputEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsInputEnabled(value).into()
        }
        unsafe extern "system" fn InputEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InputEnabled(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveInputEnabled(::core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICoreInputSourceBase, OFFSET>(),
            Dispatcher: Dispatcher::<Identity, Impl, OFFSET>,
            IsInputEnabled: IsInputEnabled::<Identity, Impl, OFFSET>,
            SetIsInputEnabled: SetIsInputEnabled::<Identity, Impl, OFFSET>,
            InputEnabled: InputEnabled::<Identity, Impl, OFFSET>,
            RemoveInputEnabled: RemoveInputEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputSourceBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ICorePointerInputSource_Impl: Sized {
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
impl ICorePointerInputSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>() -> ICorePointerInputSource_Vtbl {
        unsafe extern "system" fn ReleasePointerCapture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleasePointerCapture().into()
        }
        unsafe extern "system" fn SetPointerCapture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPointerCapture().into()
        }
        unsafe extern "system" fn HasCapture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasCapture() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerCursor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerCursor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerCursor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPointerCursor(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn PointerCaptureLost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerCaptureLost(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerCaptureLost(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerEntered<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerEntered(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerEntered(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerExited<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerExited(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerExited<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerExited(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerMoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerMoved(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerMoved(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerPressed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerPressed(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerPressed(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerReleased<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerReleased(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerReleased(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerWheelChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerWheelChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerWheelChanged(::core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICorePointerInputSource, OFFSET>(),
            ReleasePointerCapture: ReleasePointerCapture::<Identity, Impl, OFFSET>,
            SetPointerCapture: SetPointerCapture::<Identity, Impl, OFFSET>,
            HasCapture: HasCapture::<Identity, Impl, OFFSET>,
            PointerPosition: PointerPosition::<Identity, Impl, OFFSET>,
            PointerCursor: PointerCursor::<Identity, Impl, OFFSET>,
            SetPointerCursor: SetPointerCursor::<Identity, Impl, OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Identity, Impl, OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Identity, Impl, OFFSET>,
            PointerEntered: PointerEntered::<Identity, Impl, OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Identity, Impl, OFFSET>,
            PointerExited: PointerExited::<Identity, Impl, OFFSET>,
            RemovePointerExited: RemovePointerExited::<Identity, Impl, OFFSET>,
            PointerMoved: PointerMoved::<Identity, Impl, OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Identity, Impl, OFFSET>,
            PointerPressed: PointerPressed::<Identity, Impl, OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Identity, Impl, OFFSET>,
            PointerReleased: PointerReleased::<Identity, Impl, OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Identity, Impl, OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Identity, Impl, OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorePointerInputSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System"))]
pub trait ICorePointerInputSource2_Impl: Sized + ICorePointerInputSource_Impl {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::System::DispatcherQueue>;
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ::windows::core::RuntimeName for ICorePointerInputSource2 {
    const NAME: &'static str = "Windows.UI.Core.ICorePointerInputSource2";
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ICorePointerInputSource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource2_Impl, const OFFSET: isize>() -> ICorePointerInputSource2_Vtbl {
        unsafe extern "system" fn DispatcherQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerInputSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DispatcherQueue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICorePointerInputSource2, OFFSET>(),
            DispatcherQueue: DispatcherQueue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorePointerInputSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ICorePointerRedirector_Impl: Sized {
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
impl ICorePointerRedirector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>() -> ICorePointerRedirector_Vtbl {
        unsafe extern "system" fn PointerRoutedAway<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerRoutedAway(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerRoutedAway<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerRoutedAway(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerRoutedTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerRoutedTo(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerRoutedTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerRoutedTo(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerRoutedReleased<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerRoutedReleased(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerRoutedReleased<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerRoutedReleased(::core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICorePointerRedirector, OFFSET>(),
            PointerRoutedAway: PointerRoutedAway::<Identity, Impl, OFFSET>,
            RemovePointerRoutedAway: RemovePointerRoutedAway::<Identity, Impl, OFFSET>,
            PointerRoutedTo: PointerRoutedTo::<Identity, Impl, OFFSET>,
            RemovePointerRoutedTo: RemovePointerRoutedTo::<Identity, Impl, OFFSET>,
            PointerRoutedReleased: PointerRoutedReleased::<Identity, Impl, OFFSET>,
            RemovePointerRoutedReleased: RemovePointerRoutedReleased::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorePointerRedirector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "System"))]
pub trait ICoreWindow_Impl: Sized {
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
#[cfg(all(feature = "Foundation_Collections", feature = "System"))]
impl ::windows::core::RuntimeName for ICoreWindow {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindow";
}
#[cfg(all(feature = "Foundation_Collections", feature = "System"))]
impl ICoreWindow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>() -> ICoreWindow_Vtbl {
        unsafe extern "system" fn AutomationHostProvider<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutomationHostProvider() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bounds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CustomProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Dispatcher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlowDirection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut CoreWindowFlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FlowDirection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlowDirection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: CoreWindowFlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFlowDirection(value).into()
        }
        unsafe extern "system" fn IsInputEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInputEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInputEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsInputEnabled(value).into()
        }
        unsafe extern "system" fn PointerCursor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerCursor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerCursor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPointerCursor(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn PointerPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Visible<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Visible() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn GetAsyncKeyState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAsyncKeyState(virtualkey) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetKeyState(virtualkey) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleasePointerCapture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleasePointerCapture().into()
        }
        unsafe extern "system" fn SetPointerCapture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPointerCapture().into()
        }
        unsafe extern "system" fn Activated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Activated(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveActivated(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn AutomationProviderRequested<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AutomationProviderRequested(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAutomationProviderRequested<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAutomationProviderRequested(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn CharacterReceived<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CharacterReceived(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCharacterReceived<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveCharacterReceived(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn Closed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Closed(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveClosed(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn InputEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InputEnabled(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInputEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveInputEnabled(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn KeyDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KeyDown(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveKeyDown(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn KeyUp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.KeyUp(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyUp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveKeyUp(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerCaptureLost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerCaptureLost(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerCaptureLost(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerEntered<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerEntered(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerEntered(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerExited<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerExited(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerExited<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerExited(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerMoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerMoved(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerMoved(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerPressed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerPressed(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerPressed(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerReleased<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerReleased(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerReleased(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn TouchHitTesting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TouchHitTesting(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTouchHitTesting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveTouchHitTesting(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn PointerWheelChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PointerWheelChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePointerWheelChanged(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn SizeChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SizeChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSizeChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveSizeChanged(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn VisibilityChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VisibilityChanged(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisibilityChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveVisibilityChanged(::core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindow, OFFSET>(),
            AutomationHostProvider: AutomationHostProvider::<Identity, Impl, OFFSET>,
            Bounds: Bounds::<Identity, Impl, OFFSET>,
            CustomProperties: CustomProperties::<Identity, Impl, OFFSET>,
            Dispatcher: Dispatcher::<Identity, Impl, OFFSET>,
            FlowDirection: FlowDirection::<Identity, Impl, OFFSET>,
            SetFlowDirection: SetFlowDirection::<Identity, Impl, OFFSET>,
            IsInputEnabled: IsInputEnabled::<Identity, Impl, OFFSET>,
            SetIsInputEnabled: SetIsInputEnabled::<Identity, Impl, OFFSET>,
            PointerCursor: PointerCursor::<Identity, Impl, OFFSET>,
            SetPointerCursor: SetPointerCursor::<Identity, Impl, OFFSET>,
            PointerPosition: PointerPosition::<Identity, Impl, OFFSET>,
            Visible: Visible::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetAsyncKeyState: GetAsyncKeyState::<Identity, Impl, OFFSET>,
            GetKeyState: GetKeyState::<Identity, Impl, OFFSET>,
            ReleasePointerCapture: ReleasePointerCapture::<Identity, Impl, OFFSET>,
            SetPointerCapture: SetPointerCapture::<Identity, Impl, OFFSET>,
            Activated: Activated::<Identity, Impl, OFFSET>,
            RemoveActivated: RemoveActivated::<Identity, Impl, OFFSET>,
            AutomationProviderRequested: AutomationProviderRequested::<Identity, Impl, OFFSET>,
            RemoveAutomationProviderRequested: RemoveAutomationProviderRequested::<Identity, Impl, OFFSET>,
            CharacterReceived: CharacterReceived::<Identity, Impl, OFFSET>,
            RemoveCharacterReceived: RemoveCharacterReceived::<Identity, Impl, OFFSET>,
            Closed: Closed::<Identity, Impl, OFFSET>,
            RemoveClosed: RemoveClosed::<Identity, Impl, OFFSET>,
            InputEnabled: InputEnabled::<Identity, Impl, OFFSET>,
            RemoveInputEnabled: RemoveInputEnabled::<Identity, Impl, OFFSET>,
            KeyDown: KeyDown::<Identity, Impl, OFFSET>,
            RemoveKeyDown: RemoveKeyDown::<Identity, Impl, OFFSET>,
            KeyUp: KeyUp::<Identity, Impl, OFFSET>,
            RemoveKeyUp: RemoveKeyUp::<Identity, Impl, OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Identity, Impl, OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Identity, Impl, OFFSET>,
            PointerEntered: PointerEntered::<Identity, Impl, OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Identity, Impl, OFFSET>,
            PointerExited: PointerExited::<Identity, Impl, OFFSET>,
            RemovePointerExited: RemovePointerExited::<Identity, Impl, OFFSET>,
            PointerMoved: PointerMoved::<Identity, Impl, OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Identity, Impl, OFFSET>,
            PointerPressed: PointerPressed::<Identity, Impl, OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Identity, Impl, OFFSET>,
            PointerReleased: PointerReleased::<Identity, Impl, OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Identity, Impl, OFFSET>,
            TouchHitTesting: TouchHitTesting::<Identity, Impl, OFFSET>,
            RemoveTouchHitTesting: RemoveTouchHitTesting::<Identity, Impl, OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Identity, Impl, OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Identity, Impl, OFFSET>,
            SizeChanged: SizeChanged::<Identity, Impl, OFFSET>,
            RemoveSizeChanged: RemoveSizeChanged::<Identity, Impl, OFFSET>,
            VisibilityChanged: VisibilityChanged::<Identity, Impl, OFFSET>,
            RemoveVisibilityChanged: RemoveVisibilityChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindow as ::windows::core::Interface>::IID
    }
}
pub trait ICoreWindowEventArgs_Impl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICoreWindowEventArgs {
    const NAME: &'static str = "Windows.UI.Core.ICoreWindowEventArgs";
}
impl ICoreWindowEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowEventArgs_Impl, const OFFSET: isize>() -> ICoreWindowEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Handled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHandled(value).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICoreWindowEventArgs, OFFSET>(),
            Handled: Handled::<Identity, Impl, OFFSET>,
            SetHandled: SetHandled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IInitializeWithCoreWindow_Impl: Sized {
    fn Initialize(&self, window: &::core::option::Option<CoreWindow>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInitializeWithCoreWindow {
    const NAME: &'static str = "Windows.UI.Core.IInitializeWithCoreWindow";
}
impl IInitializeWithCoreWindow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInitializeWithCoreWindow_Impl, const OFFSET: isize>() -> IInitializeWithCoreWindow_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInitializeWithCoreWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&window)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IInitializeWithCoreWindow, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInitializeWithCoreWindow as ::windows::core::Interface>::IID
    }
}
