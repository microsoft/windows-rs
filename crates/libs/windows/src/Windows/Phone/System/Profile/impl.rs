#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IRetailModeStaticsImpl: Sized {
    fn RetailModeEnabled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRetailModeStatics {
    const NAME: &'static str = "Windows.Phone.System.Profile.IRetailModeStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IRetailModeStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRetailModeStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRetailModeStaticsVtbl {
        unsafe extern "system" fn RetailModeEnabled<Impl: IRetailModeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetailModeEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRetailModeStatics, BASE_OFFSET>(),
            RetailModeEnabled: RetailModeEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRetailModeStatics as ::windows::core::Interface>::IID
    }
}
