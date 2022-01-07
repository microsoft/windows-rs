pub trait IWaaSAssessorImpl: Sized {
    fn GetOSUpdateAssessment();
}
impl ::windows::core::RuntimeName for IWaaSAssessor {
    const NAME: &'static str = "Windows.Win32.System.UpdateAssessment.IWaaSAssessor";
}
impl IWaaSAssessorVtbl {
    pub const fn new<Impl: IWaaSAssessorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWaaSAssessorVtbl {
        unsafe extern "system" fn GetOSUpdateAssessment<Impl: IWaaSAssessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result: *mut OSUpdateAssessment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOSUpdateAssessment(::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWaaSAssessor>, base.5, GetOSUpdateAssessment::<Impl, OFFSET>)
    }
}
