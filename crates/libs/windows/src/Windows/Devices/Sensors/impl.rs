pub trait ISensorDataThreshold_Impl: Sized {}
impl ::windows_core::RuntimeName for ISensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ISensorDataThreshold";
}
impl ISensorDataThreshold_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorDataThreshold_Impl, const OFFSET: isize>() -> ISensorDataThreshold_Vtbl {
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ISensorDataThreshold, OFFSET>() }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ISensorDataThreshold as ::windows_core::ComInterface>::IID
    }
}
