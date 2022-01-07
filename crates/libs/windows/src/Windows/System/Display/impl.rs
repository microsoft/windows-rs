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
    pub const fn new<Impl: IDisplayRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDisplayRequestVtbl {
        unsafe extern "system" fn RequestActive<Impl: IDisplayRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RequestActive().into()
        }
        unsafe extern "system" fn RequestRelease<Impl: IDisplayRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RequestRelease().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDisplayRequest>, base.5, RequestActive::<Impl, OFFSET>, RequestRelease::<Impl, OFFSET>)
    }
}
