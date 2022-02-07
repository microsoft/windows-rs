pub trait IAdaptiveCard_Impl: Sized {
    fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IAdaptiveCard {
    const NAME: &'static str = "Windows.UI.Shell.IAdaptiveCard";
}
impl IAdaptiveCard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveCard_Impl, const OFFSET: isize>() -> IAdaptiveCard_Vtbl {
        unsafe extern "system" fn ToJson<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ToJson() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveCard, OFFSET>(), ToJson: ToJson::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveCard as ::windows::core::Interface>::IID
    }
}
pub trait IAdaptiveCardBuilderStatics_Impl: Sized {
    fn CreateAdaptiveCardFromJson(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IAdaptiveCard>;
}
impl ::windows::core::RuntimeName for IAdaptiveCardBuilderStatics {
    const NAME: &'static str = "Windows.UI.Shell.IAdaptiveCardBuilderStatics";
}
impl IAdaptiveCardBuilderStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveCardBuilderStatics_Impl, const OFFSET: isize>() -> IAdaptiveCardBuilderStatics_Vtbl {
        unsafe extern "system" fn CreateAdaptiveCardFromJson<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveCardBuilderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateAdaptiveCardFromJson(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveCardBuilderStatics, OFFSET>(),
            CreateAdaptiveCardFromJson: CreateAdaptiveCardFromJson::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveCardBuilderStatics as ::windows::core::Interface>::IID
    }
}
