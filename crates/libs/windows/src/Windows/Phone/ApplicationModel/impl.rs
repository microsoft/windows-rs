#[cfg(feature = "implement_exclusive")]
pub trait IApplicationProfileStaticsImpl: Sized {
    fn Modes(&self) -> ::windows::core::Result<ApplicationProfileModes>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationProfileStatics {
    const NAME: &'static str = "Windows.Phone.ApplicationModel.IApplicationProfileStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationProfileStaticsVtbl {
    pub const fn new<Impl: IApplicationProfileStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplicationProfileStaticsVtbl {
        unsafe extern "system" fn Modes<Impl: IApplicationProfileStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationProfileModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Modes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplicationProfileStatics>, base.5, Modes::<Impl, OFFSET>)
    }
}
