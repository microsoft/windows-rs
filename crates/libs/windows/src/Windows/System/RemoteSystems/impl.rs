#[doc = "*Required features: `\"System_RemoteSystems\"`, `\"implement\"`*"]
pub trait IRemoteSystemFilter_Impl: Sized {}
impl ::windows_core::RuntimeName for IRemoteSystemFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemFilter";
}
impl IRemoteSystemFilter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRemoteSystemFilter_Impl, const OFFSET: isize>() -> IRemoteSystemFilter_Vtbl {
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IRemoteSystemFilter, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRemoteSystemFilter as ::windows_core::ComInterface>::IID
    }
}
