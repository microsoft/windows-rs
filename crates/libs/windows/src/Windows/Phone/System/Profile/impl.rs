#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IRetailModeStaticsImpl: Sized {
    fn RetailModeEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRetailModeStatics {
    const NAME: &'static str = "Windows.Phone.System.Profile.IRetailModeStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl IRetailModeStaticsVtbl {
    pub const fn new<Impl: IRetailModeStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRetailModeStaticsVtbl {
        unsafe extern "system" fn RetailModeEnabled<Impl: IRetailModeStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RetailModeEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRetailModeStatics>, base.5, RetailModeEnabled::<Impl, OFFSET>)
    }
}
