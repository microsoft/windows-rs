pub trait IXamlDirectObject_Impl: Sized {}
impl ::windows::core::RuntimeName for IXamlDirectObject {
    const NAME: &'static str = "Windows.UI.Xaml.Core.Direct.IXamlDirectObject";
}
impl IXamlDirectObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlDirectObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlDirectObject_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlDirectObject, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlDirectObject as ::windows::core::Interface>::IID
    }
}
