pub trait IRemoteSystemFilter_Impl: Sized + windows_core::IUnknownImpl {}
impl windows_core::RuntimeName for IRemoteSystemFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemFilter";
}
impl IRemoteSystemFilter_Vtbl {
    pub const fn new<Identity: IRemoteSystemFilter_Impl, const OFFSET: isize>() -> IRemoteSystemFilter_Vtbl {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IRemoteSystemFilter, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRemoteSystemFilter as windows_core::Interface>::IID
    }
}
