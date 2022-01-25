pub trait IRemoteSystemFilter_Impl: Sized {}
impl ::windows::core::RuntimeName for IRemoteSystemFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemFilter";
}
impl IRemoteSystemFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemFilter_Impl, const OFFSET: isize>() -> IRemoteSystemFilter_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemFilter, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemFilter as ::windows::core::Interface>::IID
    }
}
