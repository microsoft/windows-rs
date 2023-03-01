#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWaaSAssessor_Impl: Sized {
    fn GetOSUpdateAssessment(&self) -> ::windows::core::Result<OSUpdateAssessment>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWaaSAssessor {}
#[cfg(feature = "Win32_Foundation")]
impl IWaaSAssessor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWaaSAssessor_Impl, const OFFSET: isize>() -> IWaaSAssessor_Vtbl {
        unsafe extern "system" fn GetOSUpdateAssessment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWaaSAssessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: *mut OSUpdateAssessment) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOSUpdateAssessment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOSUpdateAssessment: GetOSUpdateAssessment::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWaaSAssessor as ::windows::core::ComInterface>::IID
    }
}
