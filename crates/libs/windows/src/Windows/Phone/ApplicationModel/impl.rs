#[cfg(feature = "implement_exclusive")]
pub trait IApplicationProfileStaticsImpl: Sized {
    fn Modes(&mut self) -> ::windows::core::Result<ApplicationProfileModes>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationProfileStatics {
    const NAME: &'static str = "Windows.Phone.ApplicationModel.IApplicationProfileStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationProfileStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationProfileStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationProfileStaticsVtbl {
        unsafe extern "system" fn Modes<Impl: IApplicationProfileStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationProfileModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationProfileStatics, BASE_OFFSET>(), Modes: Modes::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationProfileStatics as ::windows::core::Interface>::IID
    }
}
