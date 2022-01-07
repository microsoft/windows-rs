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
    pub const fn new<Impl: IDesktopWindowTargetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDesktopWindowTargetVtbl {
        unsafe extern "system" fn IsTopmost<Impl: IDesktopWindowTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTopmost() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDesktopWindowTarget>, base.5, IsTopmost::<Impl, OFFSET>)
    }
}
