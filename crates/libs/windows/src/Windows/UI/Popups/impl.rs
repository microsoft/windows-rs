#[doc = "*Required features: `\"UI_Popups\"`, `\"implement\"`*"]
pub trait IUICommand_Impl: Sized {
    fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Invoked(&self) -> ::windows_core::Result<UICommandInvokedHandler>;
    fn SetInvoked(&self, value: ::core::option::Option<&UICommandInvokedHandler>) -> ::windows_core::Result<()>;
    fn Id(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn SetId(&self, value: ::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IUICommand {
    const NAME: &'static str = "Windows.UI.Popups.IUICommand";
}
impl IUICommand_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: isize>() -> IUICommand_Vtbl {
        unsafe extern "system" fn Label<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Label() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLabel(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Invoked<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Invoked() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInvoked<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInvoked(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Id() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetId(::windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IUICommand, OFFSET>(),
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            Invoked: Invoked::<Identity, Impl, OFFSET>,
            SetInvoked: SetInvoked::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IUICommand as ::windows_core::ComInterface>::IID
    }
}
