#[cfg(feature = "implement_exclusive")]
pub trait IInteractiveSessionStaticsImpl: Sized {
    fn IsRemote(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractiveSessionStatics {
    const NAME: &'static str = "Windows.System.RemoteDesktop.IInteractiveSessionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractiveSessionStaticsVtbl {
    pub const fn new<Impl: IInteractiveSessionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInteractiveSessionStaticsVtbl {
        unsafe extern "system" fn IsRemote<Impl: IInteractiveSessionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRemote() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInteractiveSessionStatics>, base.5, IsRemote::<Impl, OFFSET>)
    }
}
