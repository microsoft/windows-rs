pub trait IDevicePairingSettings_Impl: Sized {}
impl ::windows::core::RuntimeName for IDevicePairingSettings {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePairingSettings";
}
impl IDevicePairingSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePairingSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePairingSettings_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePairingSettings, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePairingSettings as ::windows::core::Interface>::IID
    }
}
