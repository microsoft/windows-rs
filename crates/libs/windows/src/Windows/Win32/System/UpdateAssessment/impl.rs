#[cfg(feature = "Win32_Foundation")]
pub trait IWaaSAssessor_Impl: Sized {
    fn GetOSUpdateAssessment(&self) -> ::windows::core::Result<OSUpdateAssessment>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWaaSAssessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWaaSAssessor_Impl, const OFFSET: isize>() -> IWaaSAssessor_Vtbl {
        unsafe extern "system" fn GetOSUpdateAssessment<Identity: ::windows::core::IUnknownImpl, Impl: IWaaSAssessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: *mut OSUpdateAssessment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOSUpdateAssessment() {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetOSUpdateAssessment: GetOSUpdateAssessment::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWaaSAssessor as ::windows::core::Interface>::IID
    }
}
