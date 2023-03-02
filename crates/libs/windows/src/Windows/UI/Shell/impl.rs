#[doc = "*Required features: `\"UI_Shell\"`, `\"implement\"`*"]
pub trait IAdaptiveCard_Impl: Sized {
    fn ToJson(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IAdaptiveCard {
    const NAME: &'static str = "Windows.UI.Shell.IAdaptiveCard";
}
impl IAdaptiveCard_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdaptiveCard_Impl, const OFFSET: isize>() -> IAdaptiveCard_Vtbl {
        unsafe extern "system" fn ToJson<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdaptiveCard_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ToJson() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAdaptiveCard, OFFSET>(), ToJson: ToJson::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveCard as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_Shell\"`, `\"implement\"`*"]
pub trait IAdaptiveCardBuilderStatics_Impl: Sized {
    fn CreateAdaptiveCardFromJson(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IAdaptiveCard>;
}
impl ::windows::core::RuntimeName for IAdaptiveCardBuilderStatics {
    const NAME: &'static str = "Windows.UI.Shell.IAdaptiveCardBuilderStatics";
}
impl IAdaptiveCardBuilderStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdaptiveCardBuilderStatics_Impl, const OFFSET: isize>() -> IAdaptiveCardBuilderStatics_Vtbl {
        unsafe extern "system" fn CreateAdaptiveCardFromJson<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdaptiveCardBuilderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAdaptiveCardFromJson(::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAdaptiveCardBuilderStatics, OFFSET>(),
            CreateAdaptiveCardFromJson: CreateAdaptiveCardFromJson::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveCardBuilderStatics as ::windows::core::ComInterface>::IID
    }
}
