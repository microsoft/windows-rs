#[cfg(feature = "implement_exclusive")]
pub trait IDisplayRequestImpl: Sized {
    fn RequestActive(&mut self) -> ::windows::core::Result<()>;
    fn RequestRelease(&mut self) -> ::windows::core::Result<()>;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDisplayRequest, BASE_OFFSET>(),
            RequestActive: RequestActive::<Impl, IMPL_OFFSET>,
            RequestRelease: RequestRelease::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDisplayRequest as ::windows::core::Interface>::IID
    }
}
