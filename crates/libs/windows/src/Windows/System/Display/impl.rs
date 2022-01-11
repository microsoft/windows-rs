#[cfg(feature = "implement_exclusive")]
pub trait IDisplayRequestImpl: Sized {
    fn RequestActive(&self) -> ::windows::core::Result<()>;
    fn RequestRelease(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDisplayRequest {
    const NAME: &'static str = "Windows.System.Display.IDisplayRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IDisplayRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDisplayRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDisplayRequestVtbl {
        unsafe extern "system" fn RequestActive<Impl: IDisplayRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestActive().into()
        }
        unsafe extern "system" fn RequestRelease<Impl: IDisplayRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestRelease().into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDisplayRequest>, ::windows::core::GetTrustLevel, RequestActive::<Impl, IMPL_OFFSET>, RequestRelease::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayRequest as ::windows::core::Interface>::IID
    }
}
