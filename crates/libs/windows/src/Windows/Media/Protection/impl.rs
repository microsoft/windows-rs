#[doc = "*Required features: `\"Media_Protection\"`, `\"implement\"`*"]
pub trait IMediaProtectionServiceRequest_Impl: Sized {
    fn ProtectionSystem(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn Type(&self) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::RuntimeName for IMediaProtectionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionServiceRequest";
}
impl IMediaProtectionServiceRequest_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaProtectionServiceRequest_Impl, const OFFSET: isize>() -> IMediaProtectionServiceRequest_Vtbl {
        unsafe extern "system" fn ProtectionSystem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaProtectionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProtectionSystem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMediaProtectionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IMediaProtectionServiceRequest, OFFSET>(),
            ProtectionSystem: ProtectionSystem::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMediaProtectionServiceRequest as ::windows_core::ComInterface>::IID
    }
}
