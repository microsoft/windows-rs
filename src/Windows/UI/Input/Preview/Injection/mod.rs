#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInjectedInputGamepadInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInjectedInputGamepadInfo {
    type Vtable = IInjectedInputGamepadInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(548313663, 57105, 17778, [169, 171, 215, 91, 138, 94, 72, 173]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputGamepadInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Gaming_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Gaming_Input"))] usize,
    #[cfg(feature = "Gaming_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Gaming_Input"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInjectedInputGamepadInfoFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInjectedInputGamepadInfoFactory {
    type Vtable = IInjectedInputGamepadInfoFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1499031670, 27705, 20164, [139, 42, 41, 239, 125, 225, 138, 202]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputGamepadInfoFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Gaming_Input")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reading: super::super::super::super::Gaming::Input::GamepadReading, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Gaming_Input"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInjectedInputKeyboardInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInjectedInputKeyboardInfo {
    type Vtable = IInjectedInputKeyboardInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1262932288, 11114, 24570, [126, 174, 189, 7, 123, 5, 42, 205]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputKeyboardInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InjectedInputKeyOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InjectedInputKeyOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInjectedInputMouseInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInjectedInputMouseInfo {
    type Vtable = IInjectedInputMouseInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2532666987, 58490, 23796, [65, 141, 138, 95, 185, 103, 12, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputMouseInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InjectedInputMouseOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InjectedInputMouseOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInjectedInputPenInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInjectedInputPenInfo {
    type Vtable = IInjectedInputPenInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1799400707, 51742, 21799, [126, 2, 40, 40, 84, 11, 177, 212]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputPenInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InjectedInputPointerInfo) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InjectedInputPointerInfo) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InjectedInputPenButtons) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InjectedInputPenButtons) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InjectedInputPenParameters) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InjectedInputPenParameters) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInjectedInputTouchInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInjectedInputTouchInfo {
    type Vtable = IInjectedInputTouchInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(575656415, 17384, 24309, [81, 10, 105, 202, 140, 155, 76, 40]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInjectedInputTouchInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InjectedInputRectangle) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InjectedInputRectangle) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InjectedInputPointerInfo) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InjectedInputPointerInfo) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InjectedInputTouchParameters) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InjectedInputTouchParameters) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInputInjector(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputInjector {
    type Vtable = IInputInjector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2395107204, 2818, 19410, [173, 122, 61, 70, 88, 190, 62, 24]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputInjector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visualmode: InjectedInputVisualizationMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visualmode: InjectedInputVisualizationMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shortcut: InjectedInputShortcut) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInputInjector2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputInjector2 {
    type Vtable = IInputInjector2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2390397021, 5203, 17319, [155, 203, 6, 214, 215, 179, 5, 247]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputInjector2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, input: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInputInjectorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputInjectorStatics {
    type Vtable = IInputInjectorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3735972163, 29698, 16705, [165, 198, 12, 1, 170, 87, 177, 106]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputInjectorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInputInjectorStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInputInjectorStatics2 {
    type Vtable = IInputInjectorStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2765830395, 56716, 16719, [149, 234, 248, 126, 244, 192, 174, 108]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputInjectorStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InjectedInputButtonChangeKind(pub i32);
impl InjectedInputButtonChangeKind {
    pub const None: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(0i32);
    pub const FirstButtonDown: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(1i32);
    pub const FirstButtonUp: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(2i32);
    pub const SecondButtonDown: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(3i32);
    pub const SecondButtonUp: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(4i32);
    pub const ThirdButtonDown: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(5i32);
    pub const ThirdButtonUp: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(6i32);
    pub const FourthButtonDown: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(7i32);
    pub const FourthButtonUp: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(8i32);
    pub const FifthButtonDown: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(9i32);
    pub const FifthButtonUp: InjectedInputButtonChangeKind = InjectedInputButtonChangeKind(10i32);
}
impl ::std::convert::From<i32> for InjectedInputButtonChangeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InjectedInputButtonChangeKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputButtonChangeKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputButtonChangeKind;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InjectedInputGamepadInfo(::windows::runtime::IInspectable);
impl InjectedInputGamepadInfo {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InjectedInputGamepadInfo, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Gaming_Input")]
    pub fn Buttons(&self) -> ::windows::runtime::Result<super::super::super::super::Gaming::Input::GamepadButtons> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Gaming::Input::GamepadButtons = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Gaming::Input::GamepadButtons>(result__)
        }
    }
    #[cfg(feature = "Gaming_Input")]
    pub fn SetButtons(&self, value: super::super::super::super::Gaming::Input::GamepadButtons) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn LeftThumbstickX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetLeftThumbstickX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn LeftThumbstickY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetLeftThumbstickY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn LeftTrigger(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetLeftTrigger(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn RightThumbstickX(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRightThumbstickX(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn RightThumbstickY(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRightThumbstickY(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn RightTrigger(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRightTrigger(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Gaming_Input")]
    pub fn CreateInstanceFromGamepadReading<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Gaming::Input::GamepadReading>>(reading: Param0) -> ::windows::runtime::Result<InjectedInputGamepadInfo> {
        Self::IInjectedInputGamepadInfoFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), reading.into_param().abi(), &mut result__).from_abi::<InjectedInputGamepadInfo>(result__)
        })
    }
    pub fn IInjectedInputGamepadInfoFactory<R, F: FnOnce(&IInjectedInputGamepadInfoFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InjectedInputGamepadInfo, IInjectedInputGamepadInfoFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputGamepadInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo;{20ae9a3f-df11-4572-a9ab-d75b8a5e48ad})");
}
unsafe impl ::windows::runtime::Interface for InjectedInputGamepadInfo {
    type Vtable = IInjectedInputGamepadInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(548313663, 57105, 17778, [169, 171, 215, 91, 138, 94, 72, 173]);
}
impl ::windows::runtime::RuntimeName for InjectedInputGamepadInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputGamepadInfo";
}
impl ::std::convert::From<InjectedInputGamepadInfo> for ::windows::runtime::IUnknown {
    fn from(value: InjectedInputGamepadInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InjectedInputGamepadInfo> for ::windows::runtime::IUnknown {
    fn from(value: &InjectedInputGamepadInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InjectedInputGamepadInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InjectedInputGamepadInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InjectedInputGamepadInfo> for ::windows::runtime::IInspectable {
    fn from(value: InjectedInputGamepadInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InjectedInputGamepadInfo> for ::windows::runtime::IInspectable {
    fn from(value: &InjectedInputGamepadInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InjectedInputGamepadInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InjectedInputGamepadInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InjectedInputKeyOptions(pub u32);
impl InjectedInputKeyOptions {
    pub const None: InjectedInputKeyOptions = InjectedInputKeyOptions(0u32);
    pub const ExtendedKey: InjectedInputKeyOptions = InjectedInputKeyOptions(1u32);
    pub const KeyUp: InjectedInputKeyOptions = InjectedInputKeyOptions(2u32);
    pub const ScanCode: InjectedInputKeyOptions = InjectedInputKeyOptions(8u32);
    pub const Unicode: InjectedInputKeyOptions = InjectedInputKeyOptions(4u32);
}
impl ::std::convert::From<u32> for InjectedInputKeyOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InjectedInputKeyOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputKeyOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputKeyOptions;u4)");
}
impl ::std::ops::BitOr for InjectedInputKeyOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for InjectedInputKeyOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for InjectedInputKeyOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for InjectedInputKeyOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for InjectedInputKeyOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InjectedInputKeyboardInfo(::windows::runtime::IInspectable);
impl InjectedInputKeyboardInfo {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InjectedInputKeyboardInfo, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn KeyOptions(&self) -> ::windows::runtime::Result<InjectedInputKeyOptions> {
        let this = self;
        unsafe {
            let mut result__: InjectedInputKeyOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InjectedInputKeyOptions>(result__)
        }
    }
    pub fn SetKeyOptions(&self, value: InjectedInputKeyOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScanCode(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn SetScanCode(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn VirtualKey(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn SetVirtualKey(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputKeyboardInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo;{4b46d140-2b6a-5ffa-7eae-bd077b052acd})");
}
unsafe impl ::windows::runtime::Interface for InjectedInputKeyboardInfo {
    type Vtable = IInjectedInputKeyboardInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1262932288, 11114, 24570, [126, 174, 189, 7, 123, 5, 42, 205]);
}
impl ::windows::runtime::RuntimeName for InjectedInputKeyboardInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputKeyboardInfo";
}
impl ::std::convert::From<InjectedInputKeyboardInfo> for ::windows::runtime::IUnknown {
    fn from(value: InjectedInputKeyboardInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InjectedInputKeyboardInfo> for ::windows::runtime::IUnknown {
    fn from(value: &InjectedInputKeyboardInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InjectedInputKeyboardInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InjectedInputKeyboardInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InjectedInputKeyboardInfo> for ::windows::runtime::IInspectable {
    fn from(value: InjectedInputKeyboardInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InjectedInputKeyboardInfo> for ::windows::runtime::IInspectable {
    fn from(value: &InjectedInputKeyboardInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InjectedInputKeyboardInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InjectedInputKeyboardInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InjectedInputMouseInfo(::windows::runtime::IInspectable);
impl InjectedInputMouseInfo {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InjectedInputMouseInfo, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn MouseOptions(&self) -> ::windows::runtime::Result<InjectedInputMouseOptions> {
        let this = self;
        unsafe {
            let mut result__: InjectedInputMouseOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InjectedInputMouseOptions>(result__)
        }
    }
    pub fn SetMouseOptions(&self, value: InjectedInputMouseOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn MouseData(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetMouseData(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn DeltaY(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetDeltaY(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn DeltaX(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetDeltaX(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn TimeOffsetInMilliseconds(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetTimeOffsetInMilliseconds(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputMouseInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo;{96f56e6b-e47a-5cf4-418d-8a5fb9670c7d})");
}
unsafe impl ::windows::runtime::Interface for InjectedInputMouseInfo {
    type Vtable = IInjectedInputMouseInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2532666987, 58490, 23796, [65, 141, 138, 95, 185, 103, 12, 125]);
}
impl ::windows::runtime::RuntimeName for InjectedInputMouseInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputMouseInfo";
}
impl ::std::convert::From<InjectedInputMouseInfo> for ::windows::runtime::IUnknown {
    fn from(value: InjectedInputMouseInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InjectedInputMouseInfo> for ::windows::runtime::IUnknown {
    fn from(value: &InjectedInputMouseInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InjectedInputMouseInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InjectedInputMouseInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InjectedInputMouseInfo> for ::windows::runtime::IInspectable {
    fn from(value: InjectedInputMouseInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InjectedInputMouseInfo> for ::windows::runtime::IInspectable {
    fn from(value: &InjectedInputMouseInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InjectedInputMouseInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InjectedInputMouseInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InjectedInputMouseOptions(pub u32);
impl InjectedInputMouseOptions {
    pub const None: InjectedInputMouseOptions = InjectedInputMouseOptions(0u32);
    pub const Move: InjectedInputMouseOptions = InjectedInputMouseOptions(1u32);
    pub const LeftDown: InjectedInputMouseOptions = InjectedInputMouseOptions(2u32);
    pub const LeftUp: InjectedInputMouseOptions = InjectedInputMouseOptions(4u32);
    pub const RightDown: InjectedInputMouseOptions = InjectedInputMouseOptions(8u32);
    pub const RightUp: InjectedInputMouseOptions = InjectedInputMouseOptions(16u32);
    pub const MiddleDown: InjectedInputMouseOptions = InjectedInputMouseOptions(32u32);
    pub const MiddleUp: InjectedInputMouseOptions = InjectedInputMouseOptions(64u32);
    pub const XDown: InjectedInputMouseOptions = InjectedInputMouseOptions(128u32);
    pub const XUp: InjectedInputMouseOptions = InjectedInputMouseOptions(256u32);
    pub const Wheel: InjectedInputMouseOptions = InjectedInputMouseOptions(2048u32);
    pub const HWheel: InjectedInputMouseOptions = InjectedInputMouseOptions(4096u32);
    pub const MoveNoCoalesce: InjectedInputMouseOptions = InjectedInputMouseOptions(8192u32);
    pub const VirtualDesk: InjectedInputMouseOptions = InjectedInputMouseOptions(16384u32);
    pub const Absolute: InjectedInputMouseOptions = InjectedInputMouseOptions(32768u32);
}
impl ::std::convert::From<u32> for InjectedInputMouseOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InjectedInputMouseOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputMouseOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputMouseOptions;u4)");
}
impl ::std::ops::BitOr for InjectedInputMouseOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for InjectedInputMouseOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for InjectedInputMouseOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for InjectedInputMouseOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for InjectedInputMouseOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InjectedInputPenButtons(pub u32);
impl InjectedInputPenButtons {
    pub const None: InjectedInputPenButtons = InjectedInputPenButtons(0u32);
    pub const Barrel: InjectedInputPenButtons = InjectedInputPenButtons(1u32);
    pub const Inverted: InjectedInputPenButtons = InjectedInputPenButtons(2u32);
    pub const Eraser: InjectedInputPenButtons = InjectedInputPenButtons(4u32);
}
impl ::std::convert::From<u32> for InjectedInputPenButtons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InjectedInputPenButtons {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputPenButtons {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputPenButtons;u4)");
}
impl ::std::ops::BitOr for InjectedInputPenButtons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for InjectedInputPenButtons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for InjectedInputPenButtons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for InjectedInputPenButtons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for InjectedInputPenButtons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InjectedInputPenInfo(::windows::runtime::IInspectable);
impl InjectedInputPenInfo {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InjectedInputPenInfo, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn PointerInfo(&self) -> ::windows::runtime::Result<InjectedInputPointerInfo> {
        let this = self;
        unsafe {
            let mut result__: InjectedInputPointerInfo = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InjectedInputPointerInfo>(result__)
        }
    }
    pub fn SetPointerInfo<'a, Param0: ::windows::runtime::IntoParam<'a, InjectedInputPointerInfo>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PenButtons(&self) -> ::windows::runtime::Result<InjectedInputPenButtons> {
        let this = self;
        unsafe {
            let mut result__: InjectedInputPenButtons = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InjectedInputPenButtons>(result__)
        }
    }
    pub fn SetPenButtons(&self, value: InjectedInputPenButtons) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn PenParameters(&self) -> ::windows::runtime::Result<InjectedInputPenParameters> {
        let this = self;
        unsafe {
            let mut result__: InjectedInputPenParameters = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InjectedInputPenParameters>(result__)
        }
    }
    pub fn SetPenParameters(&self, value: InjectedInputPenParameters) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Pressure(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetPressure(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Rotation(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRotation(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn TiltX(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetTiltX(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn TiltY(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetTiltY(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputPenInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputPenInfo;{6b40ad03-ca1e-5527-7e02-2828540bb1d4})");
}
unsafe impl ::windows::runtime::Interface for InjectedInputPenInfo {
    type Vtable = IInjectedInputPenInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1799400707, 51742, 21799, [126, 2, 40, 40, 84, 11, 177, 212]);
}
impl ::windows::runtime::RuntimeName for InjectedInputPenInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputPenInfo";
}
impl ::std::convert::From<InjectedInputPenInfo> for ::windows::runtime::IUnknown {
    fn from(value: InjectedInputPenInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InjectedInputPenInfo> for ::windows::runtime::IUnknown {
    fn from(value: &InjectedInputPenInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InjectedInputPenInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InjectedInputPenInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InjectedInputPenInfo> for ::windows::runtime::IInspectable {
    fn from(value: InjectedInputPenInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InjectedInputPenInfo> for ::windows::runtime::IInspectable {
    fn from(value: &InjectedInputPenInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InjectedInputPenInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InjectedInputPenInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InjectedInputPenParameters(pub u32);
impl InjectedInputPenParameters {
    pub const None: InjectedInputPenParameters = InjectedInputPenParameters(0u32);
    pub const Pressure: InjectedInputPenParameters = InjectedInputPenParameters(1u32);
    pub const Rotation: InjectedInputPenParameters = InjectedInputPenParameters(2u32);
    pub const TiltX: InjectedInputPenParameters = InjectedInputPenParameters(4u32);
    pub const TiltY: InjectedInputPenParameters = InjectedInputPenParameters(8u32);
}
impl ::std::convert::From<u32> for InjectedInputPenParameters {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InjectedInputPenParameters {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputPenParameters {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputPenParameters;u4)");
}
impl ::std::ops::BitOr for InjectedInputPenParameters {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for InjectedInputPenParameters {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for InjectedInputPenParameters {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for InjectedInputPenParameters {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for InjectedInputPenParameters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct InjectedInputPoint {
    pub PositionX: i32,
    pub PositionY: i32,
}
impl InjectedInputPoint {}
impl ::std::default::Default for InjectedInputPoint {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for InjectedInputPoint {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("InjectedInputPoint").field("PositionX", &self.PositionX).field("PositionY", &self.PositionY).finish()
    }
}
impl ::std::cmp::PartialEq for InjectedInputPoint {
    fn eq(&self, other: &Self) -> bool {
        self.PositionX == other.PositionX && self.PositionY == other.PositionY
    }
}
impl ::std::cmp::Eq for InjectedInputPoint {}
unsafe impl ::windows::runtime::Abi for InjectedInputPoint {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputPoint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Injection.InjectedInputPoint;i4;i4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct InjectedInputPointerInfo {
    pub PointerId: u32,
    pub PointerOptions: InjectedInputPointerOptions,
    pub PixelLocation: InjectedInputPoint,
    pub TimeOffsetInMilliseconds: u32,
    pub PerformanceCount: u64,
}
impl InjectedInputPointerInfo {}
impl ::std::default::Default for InjectedInputPointerInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for InjectedInputPointerInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("InjectedInputPointerInfo").field("PointerId", &self.PointerId).field("PointerOptions", &self.PointerOptions).field("PixelLocation", &self.PixelLocation).field("TimeOffsetInMilliseconds", &self.TimeOffsetInMilliseconds).field("PerformanceCount", &self.PerformanceCount).finish()
    }
}
impl ::std::cmp::PartialEq for InjectedInputPointerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.PointerId == other.PointerId && self.PointerOptions == other.PointerOptions && self.PixelLocation == other.PixelLocation && self.TimeOffsetInMilliseconds == other.TimeOffsetInMilliseconds && self.PerformanceCount == other.PerformanceCount
    }
}
impl ::std::cmp::Eq for InjectedInputPointerInfo {}
unsafe impl ::windows::runtime::Abi for InjectedInputPointerInfo {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputPointerInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Injection.InjectedInputPointerInfo;u4;enum(Windows.UI.Input.Preview.Injection.InjectedInputPointerOptions;u4);struct(Windows.UI.Input.Preview.Injection.InjectedInputPoint;i4;i4);u4;u8)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InjectedInputPointerOptions(pub u32);
impl InjectedInputPointerOptions {
    pub const None: InjectedInputPointerOptions = InjectedInputPointerOptions(0u32);
    pub const New: InjectedInputPointerOptions = InjectedInputPointerOptions(1u32);
    pub const InRange: InjectedInputPointerOptions = InjectedInputPointerOptions(2u32);
    pub const InContact: InjectedInputPointerOptions = InjectedInputPointerOptions(4u32);
    pub const FirstButton: InjectedInputPointerOptions = InjectedInputPointerOptions(16u32);
    pub const SecondButton: InjectedInputPointerOptions = InjectedInputPointerOptions(32u32);
    pub const Primary: InjectedInputPointerOptions = InjectedInputPointerOptions(8192u32);
    pub const Confidence: InjectedInputPointerOptions = InjectedInputPointerOptions(16384u32);
    pub const Canceled: InjectedInputPointerOptions = InjectedInputPointerOptions(32768u32);
    pub const PointerDown: InjectedInputPointerOptions = InjectedInputPointerOptions(65536u32);
    pub const Update: InjectedInputPointerOptions = InjectedInputPointerOptions(131072u32);
    pub const PointerUp: InjectedInputPointerOptions = InjectedInputPointerOptions(262144u32);
    pub const CaptureChanged: InjectedInputPointerOptions = InjectedInputPointerOptions(2097152u32);
}
impl ::std::convert::From<u32> for InjectedInputPointerOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InjectedInputPointerOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputPointerOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputPointerOptions;u4)");
}
impl ::std::ops::BitOr for InjectedInputPointerOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for InjectedInputPointerOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for InjectedInputPointerOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for InjectedInputPointerOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for InjectedInputPointerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct InjectedInputRectangle {
    pub Left: i32,
    pub Top: i32,
    pub Bottom: i32,
    pub Right: i32,
}
impl InjectedInputRectangle {}
impl ::std::default::Default for InjectedInputRectangle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for InjectedInputRectangle {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("InjectedInputRectangle").field("Left", &self.Left).field("Top", &self.Top).field("Bottom", &self.Bottom).field("Right", &self.Right).finish()
    }
}
impl ::std::cmp::PartialEq for InjectedInputRectangle {
    fn eq(&self, other: &Self) -> bool {
        self.Left == other.Left && self.Top == other.Top && self.Bottom == other.Bottom && self.Right == other.Right
    }
}
impl ::std::cmp::Eq for InjectedInputRectangle {}
unsafe impl ::windows::runtime::Abi for InjectedInputRectangle {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputRectangle {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Input.Preview.Injection.InjectedInputRectangle;i4;i4;i4;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InjectedInputShortcut(pub i32);
impl InjectedInputShortcut {
    pub const Back: InjectedInputShortcut = InjectedInputShortcut(0i32);
    pub const Start: InjectedInputShortcut = InjectedInputShortcut(1i32);
    pub const Search: InjectedInputShortcut = InjectedInputShortcut(2i32);
}
impl ::std::convert::From<i32> for InjectedInputShortcut {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InjectedInputShortcut {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputShortcut {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputShortcut;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InjectedInputTouchInfo(::windows::runtime::IInspectable);
impl InjectedInputTouchInfo {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InjectedInputTouchInfo, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Contact(&self) -> ::windows::runtime::Result<InjectedInputRectangle> {
        let this = self;
        unsafe {
            let mut result__: InjectedInputRectangle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InjectedInputRectangle>(result__)
        }
    }
    pub fn SetContact<'a, Param0: ::windows::runtime::IntoParam<'a, InjectedInputRectangle>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Orientation(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetOrientation(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn PointerInfo(&self) -> ::windows::runtime::Result<InjectedInputPointerInfo> {
        let this = self;
        unsafe {
            let mut result__: InjectedInputPointerInfo = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InjectedInputPointerInfo>(result__)
        }
    }
    pub fn SetPointerInfo<'a, Param0: ::windows::runtime::IntoParam<'a, InjectedInputPointerInfo>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Pressure(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetPressure(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn TouchParameters(&self) -> ::windows::runtime::Result<InjectedInputTouchParameters> {
        let this = self;
        unsafe {
            let mut result__: InjectedInputTouchParameters = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InjectedInputTouchParameters>(result__)
        }
    }
    pub fn SetTouchParameters(&self, value: InjectedInputTouchParameters) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputTouchInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo;{224fd1df-43e8-5ef5-510a-69ca8c9b4c28})");
}
unsafe impl ::windows::runtime::Interface for InjectedInputTouchInfo {
    type Vtable = IInjectedInputTouchInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(575656415, 17384, 24309, [81, 10, 105, 202, 140, 155, 76, 40]);
}
impl ::windows::runtime::RuntimeName for InjectedInputTouchInfo {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InjectedInputTouchInfo";
}
impl ::std::convert::From<InjectedInputTouchInfo> for ::windows::runtime::IUnknown {
    fn from(value: InjectedInputTouchInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InjectedInputTouchInfo> for ::windows::runtime::IUnknown {
    fn from(value: &InjectedInputTouchInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InjectedInputTouchInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InjectedInputTouchInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InjectedInputTouchInfo> for ::windows::runtime::IInspectable {
    fn from(value: InjectedInputTouchInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InjectedInputTouchInfo> for ::windows::runtime::IInspectable {
    fn from(value: &InjectedInputTouchInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InjectedInputTouchInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InjectedInputTouchInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InjectedInputTouchParameters(pub u32);
impl InjectedInputTouchParameters {
    pub const None: InjectedInputTouchParameters = InjectedInputTouchParameters(0u32);
    pub const Contact: InjectedInputTouchParameters = InjectedInputTouchParameters(1u32);
    pub const Orientation: InjectedInputTouchParameters = InjectedInputTouchParameters(2u32);
    pub const Pressure: InjectedInputTouchParameters = InjectedInputTouchParameters(4u32);
}
impl ::std::convert::From<u32> for InjectedInputTouchParameters {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InjectedInputTouchParameters {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputTouchParameters {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputTouchParameters;u4)");
}
impl ::std::ops::BitOr for InjectedInputTouchParameters {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for InjectedInputTouchParameters {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for InjectedInputTouchParameters {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for InjectedInputTouchParameters {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for InjectedInputTouchParameters {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InjectedInputVisualizationMode(pub i32);
impl InjectedInputVisualizationMode {
    pub const None: InjectedInputVisualizationMode = InjectedInputVisualizationMode(0i32);
    pub const Default: InjectedInputVisualizationMode = InjectedInputVisualizationMode(1i32);
    pub const Indirect: InjectedInputVisualizationMode = InjectedInputVisualizationMode(2i32);
}
impl ::std::convert::From<i32> for InjectedInputVisualizationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InjectedInputVisualizationMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InjectedInputVisualizationMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Preview.Injection.InjectedInputVisualizationMode;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InputInjector(::windows::runtime::IInspectable);
impl InputInjector {
    #[cfg(feature = "Foundation_Collections")]
    pub fn InjectKeyboardInput<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<InjectedInputKeyboardInfo>>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InjectMouseInput<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<InjectedInputMouseInfo>>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    pub fn InitializeTouchInjection(&self, visualmode: InjectedInputVisualizationMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), visualmode).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InjectTouchInput<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<InjectedInputTouchInfo>>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    pub fn UninitializeTouchInjection(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn InitializePenInjection(&self, visualmode: InjectedInputVisualizationMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), visualmode).ok() }
    }
    pub fn InjectPenInput<'a, Param0: ::windows::runtime::IntoParam<'a, InjectedInputPenInfo>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    pub fn UninitializePenInjection(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn InjectShortcut(&self, shortcut: InjectedInputShortcut) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), shortcut).ok() }
    }
    pub fn TryCreate() -> ::windows::runtime::Result<InputInjector> {
        Self::IInputInjectorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InputInjector>(result__)
        })
    }
    pub fn InitializeGamepadInjection(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInputInjector2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn InjectGamepadInput<'a, Param0: ::windows::runtime::IntoParam<'a, InjectedInputGamepadInfo>>(&self, input: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInputInjector2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), input.into_param().abi()).ok() }
    }
    pub fn UninitializeGamepadInjection(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInputInjector2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn TryCreateForAppBroadcastOnly() -> ::windows::runtime::Result<InputInjector> {
        Self::IInputInjectorStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InputInjector>(result__)
        })
    }
    pub fn IInputInjectorStatics<R, F: FnOnce(&IInputInjectorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InputInjector, IInputInjectorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInputInjectorStatics2<R, F: FnOnce(&IInputInjectorStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InputInjector, IInputInjectorStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InputInjector {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Preview.Injection.InputInjector;{8ec26f84-0b02-4bd2-ad7a-3d4658be3e18})");
}
unsafe impl ::windows::runtime::Interface for InputInjector {
    type Vtable = IInputInjector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2395107204, 2818, 19410, [173, 122, 61, 70, 88, 190, 62, 24]);
}
impl ::windows::runtime::RuntimeName for InputInjector {
    const NAME: &'static str = "Windows.UI.Input.Preview.Injection.InputInjector";
}
impl ::std::convert::From<InputInjector> for ::windows::runtime::IUnknown {
    fn from(value: InputInjector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InputInjector> for ::windows::runtime::IUnknown {
    fn from(value: &InputInjector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InputInjector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InputInjector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InputInjector> for ::windows::runtime::IInspectable {
    fn from(value: InputInjector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InputInjector> for ::windows::runtime::IInspectable {
    fn from(value: &InputInjector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InputInjector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InputInjector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
