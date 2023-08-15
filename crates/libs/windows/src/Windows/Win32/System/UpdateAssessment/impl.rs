#[doc = "*Required features: `\"Win32_System_UpdateAssessment\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWaaSAssessor_Impl: Sized {
    fn GetOSUpdateAssessment(&self) -> ::windows_core::Result<OSUpdateAssessment>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWaaSAssessor {}
#[cfg(feature = "Win32_Foundation")]
impl IWaaSAssessor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWaaSAssessor_Impl, const OFFSET: isize>() -> IWaaSAssessor_Vtbl {
        unsafe extern "system" fn GetOSUpdateAssessment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWaaSAssessor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: *mut OSUpdateAssessment) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOSUpdateAssessment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOSUpdateAssessment: GetOSUpdateAssessment::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWaaSAssessor as ::windows_core::ComInterface>::IID
    }
}
