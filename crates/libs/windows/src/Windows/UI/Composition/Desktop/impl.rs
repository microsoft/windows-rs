#[cfg(feature = "implement_exclusive")]
pub trait IDesktopWindowTargetImpl: Sized {
    fn IsTopmost(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDesktopWindowTarget {
    const NAME: &'static str = "Windows.UI.Composition.Desktop.IDesktopWindowTarget";
}
#[cfg(feature = "implement_exclusive")]
impl IDesktopWindowTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowTargetImpl, const OFFSET: isize>() -> IDesktopWindowTargetVtbl {
        unsafe extern "system" fn IsTopmost<Impl: IDesktopWindowTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDesktopWindowTarget>, ::windows::core::GetTrustLevel, IsTopmost::<Impl, OFFSET>)
    }
}
