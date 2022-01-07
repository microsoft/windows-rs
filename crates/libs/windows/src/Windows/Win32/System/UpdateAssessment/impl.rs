pub trait IWaaSAssessorImpl: Sized {
    fn GetOSUpdateAssessment();
}
impl ::windows::core::RuntimeName for IWaaSAssessor {
    const NAME: &'static str = "Windows.Win32.System.UpdateAssessment.IWaaSAssessor";
}
impl IWaaSAssessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWaaSAssessorImpl, const OFFSET: isize>() -> IWaaSAssessorVtbl {
        unsafe extern "system" fn GetOSUpdateAssessment<Impl: IWaaSAssessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: *mut OSUpdateAssessment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOSUpdateAssessment(::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWaaSAssessor>, ::windows::core::GetTrustLevel, GetOSUpdateAssessment::<Impl, OFFSET>)
    }
}
