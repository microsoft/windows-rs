pub trait ISensorDataThreshold_Impl: Sized {}
impl ::windows::core::RuntimeName for ISensorDataThreshold {
    const NAME: &'static str = "Windows.Devices.Sensors.ISensorDataThreshold";
}
impl ISensorDataThreshold_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISensorDataThreshold_Impl, const OFFSET: isize>() -> ISensorDataThreshold_Vtbl {
        Self { base__: ::windows::core::IInspectableVtbl::new::<Identity, ISensorDataThreshold, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorDataThreshold as ::windows::core::Interface>::IID
    }
}
