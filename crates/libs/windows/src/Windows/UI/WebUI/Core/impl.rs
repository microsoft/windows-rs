pub trait IWebUICommandBarElement_Impl: Sized {}
impl ::windows_core::RuntimeName for IWebUICommandBarElement {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarElement";
}
impl IWebUICommandBarElement_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebUICommandBarElement_Impl, const OFFSET: isize>() -> IWebUICommandBarElement_Vtbl {
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IWebUICommandBarElement, OFFSET>() }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWebUICommandBarElement as ::windows_core::ComInterface>::IID
    }
}
pub trait IWebUICommandBarIcon_Impl: Sized {}
impl ::windows_core::RuntimeName for IWebUICommandBarIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarIcon";
}
impl IWebUICommandBarIcon_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWebUICommandBarIcon_Impl, const OFFSET: isize>() -> IWebUICommandBarIcon_Vtbl {
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IWebUICommandBarIcon, OFFSET>() }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IWebUICommandBarIcon as ::windows_core::ComInterface>::IID
    }
}
