pub trait IUICommand_Impl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Invoked(&self) -> ::windows::core::Result<UICommandInvokedHandler>;
    fn SetInvoked(&self, value: &::core::option::Option<UICommandInvokedHandler>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetId(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IUICommand {
    const NAME: &'static str = "Windows.UI.Popups.IUICommand";
}
impl IUICommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUICommand_Impl, const OFFSET: isize>() -> IUICommand_Vtbl {
        unsafe extern "system" fn Label<Identity: ::windows::core::IUnknownImpl, Impl: IUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows::core::IUnknownImpl, Impl: IUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLabel(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Invoked<Identity: ::windows::core::IUnknownImpl, Impl: IUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Invoked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInvoked<Identity: ::windows::core::IUnknownImpl, Impl: IUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInvoked(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetId<Identity: ::windows::core::IUnknownImpl, Impl: IUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetId(::core::mem::transmute(&value)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUICommand, OFFSET>(),
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            Invoked: Invoked::<Identity, Impl, OFFSET>,
            SetInvoked: SetInvoked::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUICommand as ::windows::core::Interface>::IID
    }
}
