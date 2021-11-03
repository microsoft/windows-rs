#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebUICommandBar(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUICommandBar {
    type Vtable = IWebUICommandBar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2767978518, 56293, 16813, [141, 123, 20, 105, 139, 214, 145, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBar_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WebUICommandBarClosedDisplayMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: WebUICommandBarClosedDisplayMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebUICommandBarBitmapIcon(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUICommandBarBitmapIcon {
    type Vtable = IWebUICommandBarBitmapIcon_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2240761669, 2264, 19014, [129, 236, 0, 1, 91, 11, 28, 108]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarBitmapIcon_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebUICommandBarBitmapIconFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUICommandBarBitmapIconFactory {
    type Vtable = IWebUICommandBarBitmapIconFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4093106058, 30323, 17482, [190, 98, 172, 18, 211, 28, 34, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarBitmapIconFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebUICommandBarConfirmationButton(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUICommandBarConfirmationButton {
    type Vtable = IWebUICommandBarConfirmationButton_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2263319114, 58325, 20150, [178, 255, 143, 1, 138, 23, 33, 5]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarConfirmationButton_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_WebUI_Core`*"]
pub struct IWebUICommandBarElement(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUICommandBarElement {
    type Vtable = IWebUICommandBarElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3372654274, 10314, 17971, [138, 173, 99, 122, 39, 226, 130, 195]);
}
impl IWebUICommandBarElement {}
unsafe impl ::windows::runtime::RuntimeType for IWebUICommandBarElement {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{c9069ec2-284a-4633-8aad-637a27e282c3}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarElement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_WebUI_Core`*"]
pub struct IWebUICommandBarIcon(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUICommandBarIcon {
    type Vtable = IWebUICommandBarIcon_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3582420317, 8212, 17086, [150, 154, 125, 20, 202, 108, 138, 73]);
}
impl IWebUICommandBarIcon {}
unsafe impl ::windows::runtime::RuntimeType for IWebUICommandBarIcon {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{d587655d-2014-42be-969a-7d14ca6c8a49}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarIcon_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebUICommandBarIconButton(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUICommandBarIconButton {
    type Vtable = IWebUICommandBarIconButton_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2400962874, 14972, 18498, [160, 207, 175, 246, 234, 48, 133, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarIconButton_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebUICommandBarItemInvokedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUICommandBarItemInvokedEventArgs {
    type Vtable = IWebUICommandBarItemInvokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(810474461, 59201, 16879, [189, 196, 164, 92, 234, 42, 79, 112]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarItemInvokedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebUICommandBarSizeChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUICommandBarSizeChangedEventArgs {
    type Vtable = IWebUICommandBarSizeChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4226933494, 12329, 18201, [131, 120, 146, 248, 43, 135, 175, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSizeChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebUICommandBarStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUICommandBarStatics {
    type Vtable = IWebUICommandBarStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(340381113, 42246, 17854, [143, 66, 178, 131, 126, 47, 224, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebUICommandBarSymbolIcon(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUICommandBarSymbolIcon {
    type Vtable = IWebUICommandBarSymbolIcon_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3566425207, 64806, 18157, [134, 88, 26, 63, 68, 0, 231, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSymbolIcon_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebUICommandBarSymbolIconFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUICommandBarSymbolIconFactory {
    type Vtable = IWebUICommandBarSymbolIconFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1371413023, 14128, 17054, [182, 34, 20, 226, 183, 191, 106, 7]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSymbolIconFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, symbol: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_WebUI_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MenuClosedEventHandler(::windows::runtime::IUnknown);
impl MenuClosedEventHandler {
    pub fn new<F: FnMut() -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = MenuClosedEventHandler_box::<F> {
            vtable: &MenuClosedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn Invoke(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MenuClosedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({435387c8-4dd0-4c52-9489-d390ce7721d2})");
}
unsafe impl ::windows::runtime::Interface for MenuClosedEventHandler {
    type Vtable = MenuClosedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1129547720, 19920, 19538, [148, 137, 211, 144, 206, 119, 33, 210]);
}
#[repr(C)]
#[doc(hidden)]
pub struct MenuClosedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct MenuClosedEventHandler_box<F: FnMut() -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const MenuClosedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut() -> ::windows::runtime::Result<()> + 'static> MenuClosedEventHandler_box<F> {
    const VTABLE: MenuClosedEventHandler_abi = MenuClosedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<MenuClosedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
#[doc = "*Required features: `UI_WebUI_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MenuOpenedEventHandler(::windows::runtime::IUnknown);
impl MenuOpenedEventHandler {
    pub fn new<F: FnMut() -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = MenuOpenedEventHandler_box::<F> {
            vtable: &MenuOpenedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn Invoke(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MenuOpenedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({18dc0ad3-678f-4c19-8963-cc1c49a5ef9e})");
}
unsafe impl ::windows::runtime::Interface for MenuOpenedEventHandler {
    type Vtable = MenuOpenedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(417073875, 26511, 19481, [137, 99, 204, 28, 73, 165, 239, 158]);
}
#[repr(C)]
#[doc(hidden)]
pub struct MenuOpenedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct MenuOpenedEventHandler_box<F: FnMut() -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const MenuOpenedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut() -> ::windows::runtime::Result<()> + 'static> MenuOpenedEventHandler_box<F> {
    const VTABLE: MenuOpenedEventHandler_abi = MenuOpenedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<MenuOpenedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
#[doc = "*Required features: `UI_WebUI_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SizeChangedEventHandler(::windows::runtime::IUnknown);
impl SizeChangedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<WebUICommandBarSizeChangedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = SizeChangedEventHandler_box::<F> {
            vtable: &SizeChangedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, WebUICommandBarSizeChangedEventArgs>>(&self, eventargs: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), eventargs.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SizeChangedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({d49cfe3c-dd2e-4c28-b627-303a7f911af5})");
}
unsafe impl ::windows::runtime::Interface for SizeChangedEventHandler {
    type Vtable = SizeChangedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3567058492, 56622, 19496, [182, 39, 48, 58, 127, 145, 26, 245]);
}
#[repr(C)]
#[doc(hidden)]
pub struct SizeChangedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventargs: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct SizeChangedEventHandler_box<F: FnMut(&::std::option::Option<WebUICommandBarSizeChangedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const SizeChangedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<WebUICommandBarSizeChangedEventArgs>) -> ::windows::runtime::Result<()> + 'static> SizeChangedEventHandler_box<F> {
    const VTABLE: SizeChangedEventHandler_abi = SizeChangedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<SizeChangedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, eventargs: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&eventargs as *const <WebUICommandBarSizeChangedEventArgs as ::windows::runtime::Abi>::Abi as *const <WebUICommandBarSizeChangedEventArgs as ::windows::runtime::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `UI_WebUI_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WebUICommandBar(::windows::runtime::IInspectable);
impl WebUICommandBar {
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn Visible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn Opacity(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetOpacity(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn ForegroundColor(&self) -> ::windows::runtime::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetForegroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn BackgroundColor(&self) -> ::windows::runtime::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn ClosedDisplayMode(&self) -> ::windows::runtime::Result<WebUICommandBarClosedDisplayMode> {
        let this = self;
        unsafe {
            let mut result__: WebUICommandBarClosedDisplayMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebUICommandBarClosedDisplayMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetClosedDisplayMode(&self, value: WebUICommandBarClosedDisplayMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn IsOpen(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetIsOpen(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation_Collections`*"]
    pub fn PrimaryCommands(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation_Collections`*"]
    pub fn SecondaryCommands(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn MenuOpened<'a, Param0: ::windows::runtime::IntoParam<'a, MenuOpenedEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn RemoveMenuOpened<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn MenuClosed<'a, Param0: ::windows::runtime::IntoParam<'a, MenuClosedEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn RemoveMenuClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn SizeChanged<'a, Param0: ::windows::runtime::IntoParam<'a, SizeChangedEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn RemoveSizeChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<WebUICommandBar> {
        Self::IWebUICommandBarStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebUICommandBar>(result__)
        })
    }
    pub fn IWebUICommandBarStatics<R, F: FnOnce(&IWebUICommandBarStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebUICommandBar, IWebUICommandBarStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebUICommandBar {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBar;{a4fc0016-dbe5-41ad-8d7b-14698bd6911d})");
}
unsafe impl ::windows::runtime::Interface for WebUICommandBar {
    type Vtable = IWebUICommandBar_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2767978518, 56293, 16813, [141, 123, 20, 105, 139, 214, 145, 29]);
}
impl ::windows::runtime::RuntimeName for WebUICommandBar {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBar";
}
unsafe impl ::std::marker::Send for WebUICommandBar {}
unsafe impl ::std::marker::Sync for WebUICommandBar {}
#[doc = "*Required features: `UI_WebUI_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WebUICommandBarBitmapIcon(::windows::runtime::IInspectable);
impl WebUICommandBarBitmapIcon {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebUICommandBarBitmapIcon, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(uri: Param0) -> ::windows::runtime::Result<WebUICommandBarBitmapIcon> {
        Self::IWebUICommandBarBitmapIconFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<WebUICommandBarBitmapIcon>(result__)
        })
    }
    pub fn IWebUICommandBarBitmapIconFactory<R, F: FnOnce(&IWebUICommandBarBitmapIconFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebUICommandBarBitmapIcon, IWebUICommandBarBitmapIconFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebUICommandBarBitmapIcon {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarBitmapIcon;{858f4f45-08d8-4a46-81ec-00015b0b1c6c})");
}
unsafe impl ::windows::runtime::Interface for WebUICommandBarBitmapIcon {
    type Vtable = IWebUICommandBarBitmapIcon_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2240761669, 2264, 19014, [129, 236, 0, 1, 91, 11, 28, 108]);
}
impl ::windows::runtime::RuntimeName for WebUICommandBarBitmapIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarBitmapIcon";
}
impl ::std::convert::TryFrom<WebUICommandBarBitmapIcon> for IWebUICommandBarIcon {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUICommandBarBitmapIcon) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&WebUICommandBarBitmapIcon> for IWebUICommandBarIcon {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUICommandBarBitmapIcon) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebUICommandBarIcon> for WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebUICommandBarIcon> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebUICommandBarIcon> for &WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebUICommandBarIcon> {
        ::std::convert::TryInto::<IWebUICommandBarIcon>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for WebUICommandBarBitmapIcon {}
unsafe impl ::std::marker::Sync for WebUICommandBarBitmapIcon {}
#[doc = "*Required features: `UI_WebUI_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WebUICommandBarClosedDisplayMode(pub i32);
impl WebUICommandBarClosedDisplayMode {
    pub const Default: WebUICommandBarClosedDisplayMode = WebUICommandBarClosedDisplayMode(0i32);
    pub const Minimal: WebUICommandBarClosedDisplayMode = WebUICommandBarClosedDisplayMode(1i32);
    pub const Compact: WebUICommandBarClosedDisplayMode = WebUICommandBarClosedDisplayMode(2i32);
}
impl ::std::convert::From<i32> for WebUICommandBarClosedDisplayMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WebUICommandBarClosedDisplayMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WebUICommandBarClosedDisplayMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.WebUI.Core.WebUICommandBarClosedDisplayMode;i4)");
}
impl ::windows::runtime::DefaultType for WebUICommandBarClosedDisplayMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_WebUI_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WebUICommandBarConfirmationButton(::windows::runtime::IInspectable);
impl WebUICommandBarConfirmationButton {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebUICommandBarConfirmationButton, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn ItemInvoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WebUICommandBarConfirmationButton, WebUICommandBarItemInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn RemoveItemInvoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebUICommandBarConfirmationButton {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarConfirmationButton;{86e7824a-e3d5-4eb6-b2ff-8f018a172105})");
}
unsafe impl ::windows::runtime::Interface for WebUICommandBarConfirmationButton {
    type Vtable = IWebUICommandBarConfirmationButton_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2263319114, 58325, 20150, [178, 255, 143, 1, 138, 23, 33, 5]);
}
impl ::windows::runtime::RuntimeName for WebUICommandBarConfirmationButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarConfirmationButton";
}
impl ::std::convert::TryFrom<WebUICommandBarConfirmationButton> for IWebUICommandBarElement {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUICommandBarConfirmationButton) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&WebUICommandBarConfirmationButton> for IWebUICommandBarElement {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUICommandBarConfirmationButton) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebUICommandBarElement> for WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebUICommandBarElement> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebUICommandBarElement> for &WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebUICommandBarElement> {
        ::std::convert::TryInto::<IWebUICommandBarElement>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for WebUICommandBarConfirmationButton {}
unsafe impl ::std::marker::Sync for WebUICommandBarConfirmationButton {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct WebUICommandBarContract(pub u8);
#[doc = "*Required features: `UI_WebUI_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WebUICommandBarIconButton(::windows::runtime::IInspectable);
impl WebUICommandBarIconButton {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebUICommandBarIconButton, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn Enabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn Label(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetLabel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn IsToggleButton(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetIsToggleButton(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn IsChecked(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetIsChecked(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn Icon(&self) -> ::windows::runtime::Result<IWebUICommandBarIcon> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IWebUICommandBarIcon>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetIcon<'a, Param0: ::windows::runtime::IntoParam<'a, IWebUICommandBarIcon>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn ItemInvoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WebUICommandBarIconButton, WebUICommandBarItemInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn RemoveItemInvoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebUICommandBarIconButton {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarIconButton;{8f1bc93a-3a7c-4842-a0cf-aff6ea308586})");
}
unsafe impl ::windows::runtime::Interface for WebUICommandBarIconButton {
    type Vtable = IWebUICommandBarIconButton_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2400962874, 14972, 18498, [160, 207, 175, 246, 234, 48, 133, 134]);
}
impl ::windows::runtime::RuntimeName for WebUICommandBarIconButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarIconButton";
}
impl ::std::convert::TryFrom<WebUICommandBarIconButton> for IWebUICommandBarElement {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUICommandBarIconButton) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&WebUICommandBarIconButton> for IWebUICommandBarElement {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUICommandBarIconButton) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebUICommandBarElement> for WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebUICommandBarElement> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebUICommandBarElement> for &WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebUICommandBarElement> {
        ::std::convert::TryInto::<IWebUICommandBarElement>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for WebUICommandBarIconButton {}
unsafe impl ::std::marker::Sync for WebUICommandBarIconButton {}
#[doc = "*Required features: `UI_WebUI_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WebUICommandBarItemInvokedEventArgs(::windows::runtime::IInspectable);
impl WebUICommandBarItemInvokedEventArgs {
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn IsPrimaryCommand(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebUICommandBarItemInvokedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarItemInvokedEventArgs;{304edbdd-e741-41ef-bdc4-a45cea2a4f70})");
}
unsafe impl ::windows::runtime::Interface for WebUICommandBarItemInvokedEventArgs {
    type Vtable = IWebUICommandBarItemInvokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(810474461, 59201, 16879, [189, 196, 164, 92, 234, 42, 79, 112]);
}
impl ::windows::runtime::RuntimeName for WebUICommandBarItemInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarItemInvokedEventArgs";
}
unsafe impl ::std::marker::Send for WebUICommandBarItemInvokedEventArgs {}
unsafe impl ::std::marker::Sync for WebUICommandBarItemInvokedEventArgs {}
#[doc = "*Required features: `UI_WebUI_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WebUICommandBarSizeChangedEventArgs(::windows::runtime::IInspectable);
impl WebUICommandBarSizeChangedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_WebUI_Core`, `Foundation`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebUICommandBarSizeChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarSizeChangedEventArgs;{fbf1e2f6-3029-4719-8378-92f82b87af1e})");
}
unsafe impl ::windows::runtime::Interface for WebUICommandBarSizeChangedEventArgs {
    type Vtable = IWebUICommandBarSizeChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4226933494, 12329, 18201, [131, 120, 146, 248, 43, 135, 175, 30]);
}
impl ::windows::runtime::RuntimeName for WebUICommandBarSizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarSizeChangedEventArgs";
}
unsafe impl ::std::marker::Send for WebUICommandBarSizeChangedEventArgs {}
unsafe impl ::std::marker::Sync for WebUICommandBarSizeChangedEventArgs {}
#[doc = "*Required features: `UI_WebUI_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WebUICommandBarSymbolIcon(::windows::runtime::IInspectable);
impl WebUICommandBarSymbolIcon {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebUICommandBarSymbolIcon, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn Symbol(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn SetSymbol<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_WebUI_Core`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(symbol: Param0) -> ::windows::runtime::Result<WebUICommandBarSymbolIcon> {
        Self::IWebUICommandBarSymbolIconFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), symbol.into_param().abi(), &mut result__).from_abi::<WebUICommandBarSymbolIcon>(result__)
        })
    }
    pub fn IWebUICommandBarSymbolIconFactory<R, F: FnOnce(&IWebUICommandBarSymbolIconFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebUICommandBarSymbolIcon, IWebUICommandBarSymbolIconFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebUICommandBarSymbolIcon {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarSymbolIcon;{d4935477-fd26-46ed-8658-1a3f4400e7b3})");
}
unsafe impl ::windows::runtime::Interface for WebUICommandBarSymbolIcon {
    type Vtable = IWebUICommandBarSymbolIcon_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3566425207, 64806, 18157, [134, 88, 26, 63, 68, 0, 231, 179]);
}
impl ::windows::runtime::RuntimeName for WebUICommandBarSymbolIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarSymbolIcon";
}
impl ::std::convert::TryFrom<WebUICommandBarSymbolIcon> for IWebUICommandBarIcon {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUICommandBarSymbolIcon) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&WebUICommandBarSymbolIcon> for IWebUICommandBarIcon {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUICommandBarSymbolIcon) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebUICommandBarIcon> for WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebUICommandBarIcon> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebUICommandBarIcon> for &WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebUICommandBarIcon> {
        ::std::convert::TryInto::<IWebUICommandBarIcon>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for WebUICommandBarSymbolIcon {}
unsafe impl ::std::marker::Sync for WebUICommandBarSymbolIcon {}
