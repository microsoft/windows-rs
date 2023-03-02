#[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"implement\"`*"]
pub trait IWebUICommandBarElement_Impl: Sized {}
impl ::windows::core::RuntimeName for IWebUICommandBarElement {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarElement";
}
impl IWebUICommandBarElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebUICommandBarElement_Impl, const OFFSET: isize>() -> IWebUICommandBarElement_Vtbl {
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IWebUICommandBarElement, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarElement as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"implement\"`*"]
pub trait IWebUICommandBarIcon_Impl: Sized {}
impl ::windows::core::RuntimeName for IWebUICommandBarIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.IWebUICommandBarIcon";
}
impl IWebUICommandBarIcon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebUICommandBarIcon_Impl, const OFFSET: isize>() -> IWebUICommandBarIcon_Vtbl {
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IWebUICommandBarIcon, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebUICommandBarIcon as ::windows::core::ComInterface>::IID
    }
}
