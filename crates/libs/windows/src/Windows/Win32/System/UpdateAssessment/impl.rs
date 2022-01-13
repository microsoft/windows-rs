#[cfg(feature = "Win32_Foundation")]
pub trait IWaaSAssessorImpl: Sized {
    fn GetOSUpdateAssessment(&mut self) -> ::windows::core::Result<OSUpdateAssessment>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWaaSAssessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWaaSAssessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWaaSAssessorVtbl {
        unsafe extern "system" fn GetOSUpdateAssessment<Impl: IWaaSAssessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: *mut OSUpdateAssessment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOSUpdateAssessment() {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetOSUpdateAssessment: GetOSUpdateAssessment::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWaaSAssessor as ::windows::core::Interface>::IID
    }
}
