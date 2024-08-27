pub trait IDeviceEnumerationSettings_Impl: Sized + windows_core::IUnknownImpl {}
impl windows_core::RuntimeName for IDeviceEnumerationSettings {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDeviceEnumerationSettings";
}
impl IDeviceEnumerationSettings_Vtbl {
    pub const fn new<Identity: IDeviceEnumerationSettings_Impl, const OFFSET: isize>() -> IDeviceEnumerationSettings_Vtbl {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDeviceEnumerationSettings, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceEnumerationSettings as windows_core::Interface>::IID
    }
}
pub trait IDevicePairingSettings_Impl: Sized + windows_core::IUnknownImpl {}
impl windows_core::RuntimeName for IDevicePairingSettings {
    const NAME: &'static str = "Windows.Devices.Enumeration.IDevicePairingSettings";
}
impl IDevicePairingSettings_Vtbl {
    pub const fn new<Identity: IDevicePairingSettings_Impl, const OFFSET: isize>() -> IDevicePairingSettings_Vtbl {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDevicePairingSettings, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDevicePairingSettings as windows_core::Interface>::IID
    }
}
