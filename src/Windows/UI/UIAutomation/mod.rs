#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "UI_UIAutomation_Core")]
pub mod Core;
#[doc = "*Required features: `UI_UIAutomation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AutomationConnection(::windows::runtime::IInspectable);
impl AutomationConnection {
    #[doc = "*Required features: `UI_UIAutomation`*"]
    pub fn IsRemoteSystem(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation`*"]
    pub fn AppUserModelId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation`*"]
    pub fn ExecutableFileName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AutomationConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationConnection;{aad262ed-0ef4-5d43-97be-a834e27b65b9})");
}
unsafe impl ::windows::runtime::Interface for AutomationConnection {
    type Vtable = IAutomationConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2865914605, 3828, 23875, [151, 190, 168, 52, 226, 123, 101, 185]);
}
impl ::windows::runtime::RuntimeName for AutomationConnection {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationConnection";
}
unsafe impl ::std::marker::Send for AutomationConnection {}
unsafe impl ::std::marker::Sync for AutomationConnection {}
#[doc = "*Required features: `UI_UIAutomation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AutomationConnectionBoundObject(::windows::runtime::IInspectable);
impl AutomationConnectionBoundObject {
    #[doc = "*Required features: `UI_UIAutomation`*"]
    pub fn Connection(&self) -> ::windows::runtime::Result<AutomationConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AutomationConnection>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AutomationConnectionBoundObject {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationConnectionBoundObject;{5e8558fb-ca52-5b65-9830-dd2905816093})");
}
unsafe impl ::windows::runtime::Interface for AutomationConnectionBoundObject {
    type Vtable = IAutomationConnectionBoundObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1585797371, 51794, 23397, [152, 48, 221, 41, 5, 129, 96, 147]);
}
impl ::windows::runtime::RuntimeName for AutomationConnectionBoundObject {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationConnectionBoundObject";
}
unsafe impl ::std::marker::Send for AutomationConnectionBoundObject {}
unsafe impl ::std::marker::Sync for AutomationConnectionBoundObject {}
#[doc = "*Required features: `UI_UIAutomation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AutomationElement(::windows::runtime::IInspectable);
impl AutomationElement {
    #[doc = "*Required features: `UI_UIAutomation`*"]
    pub fn IsRemoteSystem(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation`*"]
    pub fn AppUserModelId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_UIAutomation`*"]
    pub fn ExecutableFileName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AutomationElement {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationElement;{a1898370-2c07-56fd-993f-61a72a08058c})");
}
unsafe impl ::windows::runtime::Interface for AutomationElement {
    type Vtable = IAutomationElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2710143856, 11271, 22269, [153, 63, 97, 167, 42, 8, 5, 140]);
}
impl ::windows::runtime::RuntimeName for AutomationElement {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationElement";
}
unsafe impl ::std::marker::Send for AutomationElement {}
unsafe impl ::std::marker::Sync for AutomationElement {}
#[doc = "*Required features: `UI_UIAutomation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct AutomationTextRange(::windows::runtime::IInspectable);
impl AutomationTextRange {}
unsafe impl ::windows::runtime::RuntimeType for AutomationTextRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationTextRange;{7e101b65-40d3-5994-85a9-0a0cb9a4ec98})");
}
unsafe impl ::windows::runtime::Interface for AutomationTextRange {
    type Vtable = IAutomationTextRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2114984805, 16595, 22932, [133, 169, 10, 12, 185, 164, 236, 152]);
}
impl ::windows::runtime::RuntimeName for AutomationTextRange {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationTextRange";
}
unsafe impl ::std::marker::Send for AutomationTextRange {}
unsafe impl ::std::marker::Sync for AutomationTextRange {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAutomationConnection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationConnection {
    type Vtable = IAutomationConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2865914605, 3828, 23875, [151, 190, 168, 52, 226, 123, 101, 185]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAutomationConnectionBoundObject(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationConnectionBoundObject {
    type Vtable = IAutomationConnectionBoundObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1585797371, 51794, 23397, [152, 48, 221, 41, 5, 129, 96, 147]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationConnectionBoundObject_abi(
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
pub struct IAutomationElement(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationElement {
    type Vtable = IAutomationElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2710143856, 11271, 22269, [153, 63, 97, 167, 42, 8, 5, 140]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IAutomationTextRange(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAutomationTextRange {
    type Vtable = IAutomationTextRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2114984805, 16595, 22932, [133, 169, 10, 12, 185, 164, 236, 152]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationTextRange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct UIAutomationContract(pub u8);
