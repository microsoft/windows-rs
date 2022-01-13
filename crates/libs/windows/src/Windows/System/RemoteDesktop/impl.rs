#[cfg(feature = "implement_exclusive")]
pub trait IInteractiveSessionStaticsImpl: Sized {
    fn IsRemote(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInteractiveSessionStatics {
    const NAME: &'static str = "Windows.System.RemoteDesktop.IInteractiveSessionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IInteractiveSessionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInteractiveSessionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInteractiveSessionStaticsVtbl {
        unsafe extern "system" fn IsRemote<Impl: IInteractiveSessionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRemote() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInteractiveSessionStatics, BASE_OFFSET>(), IsRemote: IsRemote::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInteractiveSessionStatics as ::windows::core::Interface>::IID
    }
}
