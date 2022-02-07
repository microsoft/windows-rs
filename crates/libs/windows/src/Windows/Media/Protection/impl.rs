pub trait IMediaProtectionServiceRequest_Impl: Sized {
    fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Type(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IMediaProtectionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionServiceRequest";
}
impl IMediaProtectionServiceRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionServiceRequest_Impl, const OFFSET: isize>() -> IMediaProtectionServiceRequest_Vtbl {
        unsafe extern "system" fn ProtectionSystem<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProtectionSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaProtectionServiceRequest, OFFSET>(),
            ProtectionSystem: ProtectionSystem::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaProtectionServiceRequest as ::windows::core::Interface>::IID
    }
}
