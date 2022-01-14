#[cfg(feature = "implement_exclusive")]
pub trait IDesktopWindowTarget_Impl: Sized {
    fn IsTopmost(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDesktopWindowTarget {
    const NAME: &'static str = "Windows.UI.Composition.Desktop.IDesktopWindowTarget";
}
#[cfg(feature = "implement_exclusive")]
impl IDesktopWindowTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowTarget_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesktopWindowTarget_Vtbl {
        unsafe extern "system" fn IsTopmost<Impl: IDesktopWindowTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDesktopWindowTarget, BASE_OFFSET>(), IsTopmost: IsTopmost::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesktopWindowTarget as ::windows::core::Interface>::IID
    }
}
