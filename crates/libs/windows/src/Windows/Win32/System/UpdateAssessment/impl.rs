#[cfg(feature = "Win32_Foundation")]
pub trait IWaaSAssessorImpl: Sized {
    fn GetOSUpdateAssessment();
}
#[cfg(feature = "Win32_Foundation")]
impl IWaaSAssessorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWaaSAssessorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWaaSAssessorVtbl {
        unsafe extern "system" fn GetOSUpdateAssessment<Impl: IWaaSAssessorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: *mut OSUpdateAssessment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetOSUpdateAssessment::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWaaSAssessor as ::windows::core::Interface>::IID
    }
}
