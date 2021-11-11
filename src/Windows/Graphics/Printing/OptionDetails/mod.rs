#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintBindingOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintBindingOptionDetails {
    type Vtable = IPrintBindingOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3f4cc98_9564_4f16_a055_a98b9a49e9d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintBindingOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintBorderingOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintBorderingOptionDetails {
    type Vtable = IPrintBorderingOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d73bc8f_fb53_4eb2_985f_1d91de0b7639);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintBorderingOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintCollationOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintCollationOptionDetails {
    type Vtable = IPrintCollationOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6abb166_a5a6_40dc_acc3_739f28f1e5d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCollationOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintColorModeOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintColorModeOptionDetails {
    type Vtable = IPrintColorModeOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdba97704_f1d6_4843_a484_9b447cdcf3b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintColorModeOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintCopiesOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintCopiesOptionDetails {
    type Vtable = IPrintCopiesOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42053099_4339_4343_898d_2c47b5e0c341);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCopiesOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintCustomItemDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintCustomItemDetails {
    type Vtable = IPrintCustomItemDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5704b637_5c3a_449a_aa36_b3291b1192fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomItemDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintCustomItemListOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintCustomItemListOptionDetails {
    type Vtable = IPrintCustomItemListOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5fafd88_58f2_4ebd_b90f_51e4f2944c5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomItemListOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itemid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintCustomItemListOptionDetails2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintCustomItemListOptionDetails2 {
    type Vtable = IPrintCustomItemListOptionDetails2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9d6353d_651c_4a39_906e_1091a1801bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomItemListOptionDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, itemid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, description: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, icon: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintCustomItemListOptionDetails3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintCustomItemListOptionDetails3 {
    type Vtable = IPrintCustomItemListOptionDetails3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fa1b53f_3c34_4868_a407_fc5eab259b21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomItemListOptionDetails3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
pub struct IPrintCustomOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintCustomOptionDetails {
    type Vtable = IPrintCustomOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe32bde1c_28af_4b90_95da_a3acf320b929);
}
impl IPrintCustomOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintCustomOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e32bde1c-28af-4b90-95da-a3acf320b929}");
}
impl ::core::convert::From<IPrintCustomOptionDetails> for ::windows::core::IUnknown {
    fn from(value: IPrintCustomOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintCustomOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &IPrintCustomOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintCustomOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintCustomOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintCustomOptionDetails> for ::windows::core::IInspectable {
    fn from(value: IPrintCustomOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintCustomOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &IPrintCustomOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrintCustomOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrintCustomOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPrintCustomOptionDetails> for IPrintOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrintCustomOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrintCustomOptionDetails> for IPrintOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrintCustomOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for IPrintCustomOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &IPrintCustomOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::core::convert::TryInto::<IPrintOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintCustomTextOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintCustomTextOptionDetails {
    type Vtable = IPrintCustomTextOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ad171f8_c8bd_4905_9192_0d75136e8b31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomTextOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintCustomTextOptionDetails2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintCustomTextOptionDetails2 {
    type Vtable = IPrintCustomTextOptionDetails2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcea70b54_b977_4718_8338_7ed2b0d86fe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomTextOptionDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintCustomToggleOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintCustomToggleOptionDetails {
    type Vtable = IPrintCustomToggleOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9db4d514_e461_4608_8ee9_db6f5ed073c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomToggleOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintDuplexOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintDuplexOptionDetails {
    type Vtable = IPrintDuplexOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcd94591_d4a4_44fa_b3fe_42e0ba28d5ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDuplexOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintHolePunchOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintHolePunchOptionDetails {
    type Vtable = IPrintHolePunchOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6de1f18_482c_4657_9d71_8ddddbea1e1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintHolePunchOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
pub struct IPrintItemListOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintItemListOptionDetails {
    type Vtable = IPrintItemListOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a2257bf_fe61_43d8_a24f_a3f6ab7320e7);
}
impl IPrintItemListOptionDetails {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintItemListOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9a2257bf-fe61-43d8-a24f-a3f6ab7320e7}");
}
impl ::core::convert::From<IPrintItemListOptionDetails> for ::windows::core::IUnknown {
    fn from(value: IPrintItemListOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintItemListOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &IPrintItemListOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintItemListOptionDetails> for ::windows::core::IInspectable {
    fn from(value: IPrintItemListOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintItemListOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &IPrintItemListOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrintItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrintItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPrintItemListOptionDetails> for IPrintOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrintItemListOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrintItemListOptionDetails> for IPrintOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrintItemListOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for IPrintItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &IPrintItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::core::convert::TryInto::<IPrintOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintItemListOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintMediaSizeOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintMediaSizeOptionDetails {
    type Vtable = IPrintMediaSizeOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c8d5bcf_c0bf_47c8_b84a_628e7d0d1a1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintMediaSizeOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintMediaTypeOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintMediaTypeOptionDetails {
    type Vtable = IPrintMediaTypeOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8c7000b_abf3_4abc_8e86_22abc5744a43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintMediaTypeOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
pub struct IPrintNumberOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintNumberOptionDetails {
    type Vtable = IPrintNumberOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d01bbaf_645c_4de9_965f_6fc6bbc47cab);
}
impl IPrintNumberOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn MinValue(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn MaxValue(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintNumberOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4d01bbaf-645c-4de9-965f-6fc6bbc47cab}");
}
impl ::core::convert::From<IPrintNumberOptionDetails> for ::windows::core::IUnknown {
    fn from(value: IPrintNumberOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintNumberOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &IPrintNumberOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintNumberOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintNumberOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintNumberOptionDetails> for ::windows::core::IInspectable {
    fn from(value: IPrintNumberOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintNumberOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &IPrintNumberOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrintNumberOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrintNumberOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPrintNumberOptionDetails> for IPrintOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrintNumberOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrintNumberOptionDetails> for IPrintOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrintNumberOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for IPrintNumberOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &IPrintNumberOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::core::convert::TryInto::<IPrintOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintNumberOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
pub struct IPrintOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl IPrintOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{390686cf-d682-495f-adfe-d7333f5c1808}");
}
impl ::core::convert::From<IPrintOptionDetails> for ::windows::core::IUnknown {
    fn from(value: IPrintOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &IPrintOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintOptionDetails> for ::windows::core::IInspectable {
    fn from(value: IPrintOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &IPrintOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrintOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrintOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintOptionType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PrintOptionStates) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PrintOptionStates) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintOrientationOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintOrientationOptionDetails {
    type Vtable = IPrintOrientationOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46c38879_66e0_4da0_87b4_d25457824eb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOrientationOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintPageRangeOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintPageRangeOptionDetails {
    type Vtable = IPrintPageRangeOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a19e4b7_2be8_4aa7_9ea5_defbe8713b4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageRangeOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintQualityOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintQualityOptionDetails {
    type Vtable = IPrintQualityOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dd06ba1_ce1a_44e6_84f9_3a92ea1e3044);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintQualityOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintStapleOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintStapleOptionDetails {
    type Vtable = IPrintStapleOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd43175bd_9c0b_44e0_84f6_ceebce653800);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintStapleOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskOptionChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskOptionChangedEventArgs {
    type Vtable = IPrintTaskOptionChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65197d05_a5ee_4307_9407_9acad147679c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskOptionDetails {
    type Vtable = IPrintTaskOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5720af1_a89e_42a6_81af_f8e010b38a68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, optionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, optionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskOptionDetails2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskOptionDetails2 {
    type Vtable = IPrintTaskOptionDetails2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53730a09_f968_4692_a177_c074597186db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, optionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintTaskOptionDetailsStatic(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTaskOptionDetailsStatic {
    type Vtable = IPrintTaskOptionDetailsStatic_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x135da193_0961_4b6e_8766_f13b7fbccd58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionDetailsStatic_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, printtaskoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
pub struct IPrintTextOptionDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintTextOptionDetails {
    type Vtable = IPrintTextOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad75e563_5ce4_46bc_9918_ab9fad144c5b);
}
impl IPrintTextOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn MaxCharacters(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPrintTextOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ad75e563-5ce4-46bc-9918-ab9fad144c5b}");
}
impl ::core::convert::From<IPrintTextOptionDetails> for ::windows::core::IUnknown {
    fn from(value: IPrintTextOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPrintTextOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &IPrintTextOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPrintTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPrintTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPrintTextOptionDetails> for ::windows::core::IInspectable {
    fn from(value: IPrintTextOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPrintTextOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &IPrintTextOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPrintTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPrintTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IPrintTextOptionDetails> for IPrintOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: IPrintTextOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IPrintTextOptionDetails> for IPrintOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &IPrintTextOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for IPrintTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &IPrintTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::core::convert::TryInto::<IPrintOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTextOptionDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintBindingOptionDetails(pub ::windows::core::IInspectable);
impl PrintBindingOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintBindingOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintBindingOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintBindingOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintBindingOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintBindingOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintBindingOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintBindingOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintBindingOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintBindingOptionDetails";
}
impl ::core::convert::From<PrintBindingOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintBindingOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintBindingOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintBindingOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintBindingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintBindingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintBindingOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintBindingOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintBindingOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintBindingOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintBindingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintBindingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintBindingOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintBindingOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintBindingOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintBindingOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintBindingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintBindingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintBindingOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintBindingOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintBindingOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintBindingOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for PrintBindingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for &PrintBindingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::core::convert::TryInto::<IPrintItemListOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintBindingOptionDetails {}
unsafe impl ::core::marker::Sync for PrintBindingOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintBorderingOptionDetails(pub ::windows::core::IInspectable);
impl PrintBorderingOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintBorderingOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintBorderingOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintBorderingOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintBorderingOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintBorderingOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintBorderingOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintBorderingOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintBorderingOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintBorderingOptionDetails";
}
impl ::core::convert::From<PrintBorderingOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintBorderingOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintBorderingOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintBorderingOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintBorderingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintBorderingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintBorderingOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintBorderingOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintBorderingOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintBorderingOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintBorderingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintBorderingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintBorderingOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintBorderingOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintBorderingOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintBorderingOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintBorderingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintBorderingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintBorderingOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintBorderingOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintBorderingOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintBorderingOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for PrintBorderingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for &PrintBorderingOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::core::convert::TryInto::<IPrintItemListOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintBorderingOptionDetails {}
unsafe impl ::core::marker::Sync for PrintBorderingOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintCollationOptionDetails(pub ::windows::core::IInspectable);
impl PrintCollationOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCollationOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCollationOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCollationOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCollationOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintCollationOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintCollationOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintCollationOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintCollationOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCollationOptionDetails";
}
impl ::core::convert::From<PrintCollationOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintCollationOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintCollationOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintCollationOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintCollationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintCollationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintCollationOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintCollationOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintCollationOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintCollationOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintCollationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintCollationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintCollationOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintCollationOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintCollationOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintCollationOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintCollationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintCollationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintCollationOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintCollationOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintCollationOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintCollationOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for PrintCollationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for &PrintCollationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::core::convert::TryInto::<IPrintItemListOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintCollationOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCollationOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintColorModeOptionDetails(pub ::windows::core::IInspectable);
impl PrintColorModeOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintColorModeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintColorModeOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintColorModeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintColorModeOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintColorModeOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintColorModeOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintColorModeOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintColorModeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintColorModeOptionDetails";
}
impl ::core::convert::From<PrintColorModeOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintColorModeOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintColorModeOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintColorModeOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintColorModeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintColorModeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintColorModeOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintColorModeOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintColorModeOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintColorModeOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintColorModeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintColorModeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintColorModeOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintColorModeOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintColorModeOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintColorModeOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintColorModeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintColorModeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintColorModeOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintColorModeOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintColorModeOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintColorModeOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for PrintColorModeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for &PrintColorModeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::core::convert::TryInto::<IPrintItemListOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintColorModeOptionDetails {}
unsafe impl ::core::marker::Sync for PrintColorModeOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintCopiesOptionDetails(pub ::windows::core::IInspectable);
impl PrintCopiesOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn MinValue(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPrintNumberOptionDetails>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn MaxValue(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPrintNumberOptionDetails>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCopiesOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCopiesOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCopiesOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCopiesOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintCopiesOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintCopiesOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintCopiesOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintCopiesOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCopiesOptionDetails";
}
impl ::core::convert::From<PrintCopiesOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintCopiesOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintCopiesOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintCopiesOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintCopiesOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintCopiesOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintCopiesOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintCopiesOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintCopiesOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintCopiesOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintCopiesOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintCopiesOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintCopiesOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintCopiesOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintCopiesOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintCopiesOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintCopiesOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintCopiesOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintCopiesOptionDetails> for IPrintNumberOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintCopiesOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintCopiesOptionDetails> for IPrintNumberOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintCopiesOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintNumberOptionDetails> for PrintCopiesOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintNumberOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintNumberOptionDetails> for &PrintCopiesOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintNumberOptionDetails> {
        ::core::convert::TryInto::<IPrintNumberOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintCopiesOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCopiesOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintCustomItemDetails(pub ::windows::core::IInspectable);
impl PrintCustomItemDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetItemDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ItemDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintCustomItemDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintCustomItemDetails;{5704b637-5c3a-449a-aa36-b3291b1192fd})");
}
unsafe impl ::windows::core::Interface for PrintCustomItemDetails {
    type Vtable = IPrintCustomItemDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5704b637_5c3a_449a_aa36_b3291b1192fd);
}
impl ::windows::core::RuntimeName for PrintCustomItemDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCustomItemDetails";
}
impl ::core::convert::From<PrintCustomItemDetails> for ::windows::core::IUnknown {
    fn from(value: PrintCustomItemDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintCustomItemDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintCustomItemDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintCustomItemDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintCustomItemDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintCustomItemDetails> for ::windows::core::IInspectable {
    fn from(value: PrintCustomItemDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintCustomItemDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintCustomItemDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintCustomItemDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintCustomItemDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintCustomItemDetails {}
unsafe impl ::core::marker::Sync for PrintCustomItemDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintCustomItemListOptionDetails(pub ::windows::core::IInspectable);
impl PrintCustomItemListOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn AddItem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, itemid: Param0, displayname: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCustomItemListOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), itemid.into_param().abi(), displayname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Storage_Streams`*"]
    pub fn AddItem2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param3: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(&self, itemid: Param0, displayname: Param1, description: Param2, icon: Param3) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCustomItemListOptionDetails2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), itemid.into_param().abi(), displayname.into_param().abi(), description.into_param().abi(), icon.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCustomItemListOptionDetails3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCustomItemListOptionDetails3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCustomItemListOptionDetails3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCustomItemListOptionDetails3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintCustomItemListOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintCustomItemListOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintCustomItemListOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails";
}
impl ::core::convert::From<PrintCustomItemListOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintCustomItemListOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintCustomItemListOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintCustomItemListOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintCustomItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintCustomItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintCustomItemListOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintCustomItemListOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintCustomItemListOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintCustomItemListOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintCustomItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintCustomItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintCustomItemListOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintCustomItemListOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintCustomItemListOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintCustomItemListOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintCustomItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintCustomItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintCustomItemListOptionDetails> for IPrintCustomOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintCustomItemListOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintCustomItemListOptionDetails> for IPrintCustomOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintCustomItemListOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintCustomOptionDetails> for PrintCustomItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintCustomOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintCustomOptionDetails> for &PrintCustomItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintCustomOptionDetails> {
        ::core::convert::TryInto::<IPrintCustomOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PrintCustomItemListOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintCustomItemListOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintCustomItemListOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintCustomItemListOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for PrintCustomItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for &PrintCustomItemListOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::core::convert::TryInto::<IPrintItemListOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintCustomItemListOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCustomItemListOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintCustomTextOptionDetails(pub ::windows::core::IInspectable);
impl PrintCustomTextOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetMaxCharacters(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCustomTextOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn MaxCharacters(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPrintCustomTextOptionDetails>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCustomTextOptionDetails2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCustomTextOptionDetails2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCustomTextOptionDetails2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCustomTextOptionDetails2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintCustomTextOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintCustomTextOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintCustomTextOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails";
}
impl ::core::convert::From<PrintCustomTextOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintCustomTextOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintCustomTextOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintCustomTextOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintCustomTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintCustomTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintCustomTextOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintCustomTextOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintCustomTextOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintCustomTextOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintCustomTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintCustomTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintCustomTextOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintCustomTextOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintCustomTextOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintCustomTextOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintCustomTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintCustomTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintCustomTextOptionDetails> for IPrintCustomOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintCustomTextOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintCustomTextOptionDetails> for IPrintCustomOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintCustomTextOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintCustomOptionDetails> for PrintCustomTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintCustomOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintCustomOptionDetails> for &PrintCustomTextOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintCustomOptionDetails> {
        ::core::convert::TryInto::<IPrintCustomOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintCustomTextOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCustomTextOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintCustomToggleOptionDetails(pub ::windows::core::IInspectable);
impl PrintCustomToggleOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCustomToggleOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCustomToggleOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintCustomToggleOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintCustomToggleOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintCustomToggleOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintCustomToggleOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintCustomToggleOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintCustomToggleOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCustomToggleOptionDetails";
}
impl ::core::convert::From<PrintCustomToggleOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintCustomToggleOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintCustomToggleOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintCustomToggleOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintCustomToggleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintCustomToggleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintCustomToggleOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintCustomToggleOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintCustomToggleOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintCustomToggleOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintCustomToggleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintCustomToggleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintCustomToggleOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintCustomToggleOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintCustomToggleOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintCustomToggleOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintCustomToggleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintCustomToggleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintCustomToggleOptionDetails> for IPrintCustomOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintCustomToggleOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintCustomToggleOptionDetails> for IPrintCustomOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintCustomToggleOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintCustomOptionDetails> for PrintCustomToggleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintCustomOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintCustomOptionDetails> for &PrintCustomToggleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintCustomOptionDetails> {
        ::core::convert::TryInto::<IPrintCustomOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintCustomToggleOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCustomToggleOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintDuplexOptionDetails(pub ::windows::core::IInspectable);
impl PrintDuplexOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintDuplexOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintDuplexOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintDuplexOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintDuplexOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintDuplexOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintDuplexOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintDuplexOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintDuplexOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintDuplexOptionDetails";
}
impl ::core::convert::From<PrintDuplexOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintDuplexOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintDuplexOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintDuplexOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintDuplexOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintDuplexOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintDuplexOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintDuplexOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintDuplexOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintDuplexOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintDuplexOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintDuplexOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintDuplexOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintDuplexOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintDuplexOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintDuplexOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintDuplexOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintDuplexOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintDuplexOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintDuplexOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintDuplexOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintDuplexOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for PrintDuplexOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for &PrintDuplexOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::core::convert::TryInto::<IPrintItemListOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintDuplexOptionDetails {}
unsafe impl ::core::marker::Sync for PrintDuplexOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintHolePunchOptionDetails(pub ::windows::core::IInspectable);
impl PrintHolePunchOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintHolePunchOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintHolePunchOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintHolePunchOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintHolePunchOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintHolePunchOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintHolePunchOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintHolePunchOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintHolePunchOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintHolePunchOptionDetails";
}
impl ::core::convert::From<PrintHolePunchOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintHolePunchOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintHolePunchOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintHolePunchOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintHolePunchOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintHolePunchOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintHolePunchOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintHolePunchOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintHolePunchOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintHolePunchOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintHolePunchOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintHolePunchOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintHolePunchOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintHolePunchOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintHolePunchOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintHolePunchOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintHolePunchOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintHolePunchOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintHolePunchOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintHolePunchOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintHolePunchOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintHolePunchOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for PrintHolePunchOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for &PrintHolePunchOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::core::convert::TryInto::<IPrintItemListOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintHolePunchOptionDetails {}
unsafe impl ::core::marker::Sync for PrintHolePunchOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintMediaSizeOptionDetails(pub ::windows::core::IInspectable);
impl PrintMediaSizeOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintMediaSizeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintMediaSizeOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintMediaSizeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintMediaSizeOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintMediaSizeOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintMediaSizeOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintMediaSizeOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintMediaSizeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintMediaSizeOptionDetails";
}
impl ::core::convert::From<PrintMediaSizeOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintMediaSizeOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintMediaSizeOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintMediaSizeOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintMediaSizeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintMediaSizeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintMediaSizeOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintMediaSizeOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintMediaSizeOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintMediaSizeOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintMediaSizeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintMediaSizeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintMediaSizeOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintMediaSizeOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintMediaSizeOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintMediaSizeOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintMediaSizeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintMediaSizeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintMediaSizeOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintMediaSizeOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintMediaSizeOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintMediaSizeOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for PrintMediaSizeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for &PrintMediaSizeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::core::convert::TryInto::<IPrintItemListOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintMediaSizeOptionDetails {}
unsafe impl ::core::marker::Sync for PrintMediaSizeOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintMediaTypeOptionDetails(pub ::windows::core::IInspectable);
impl PrintMediaTypeOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintMediaTypeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintMediaTypeOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintMediaTypeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintMediaTypeOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintMediaTypeOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintMediaTypeOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintMediaTypeOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintMediaTypeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintMediaTypeOptionDetails";
}
impl ::core::convert::From<PrintMediaTypeOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintMediaTypeOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintMediaTypeOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintMediaTypeOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintMediaTypeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintMediaTypeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintMediaTypeOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintMediaTypeOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintMediaTypeOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintMediaTypeOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintMediaTypeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintMediaTypeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintMediaTypeOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintMediaTypeOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintMediaTypeOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintMediaTypeOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintMediaTypeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintMediaTypeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintMediaTypeOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintMediaTypeOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintMediaTypeOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintMediaTypeOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for PrintMediaTypeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for &PrintMediaTypeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::core::convert::TryInto::<IPrintItemListOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintMediaTypeOptionDetails {}
unsafe impl ::core::marker::Sync for PrintMediaTypeOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintOptionStates(pub u32);
impl PrintOptionStates {
    pub const None: PrintOptionStates = PrintOptionStates(0u32);
    pub const Enabled: PrintOptionStates = PrintOptionStates(1u32);
    pub const Constrained: PrintOptionStates = PrintOptionStates(2u32);
}
impl ::core::convert::From<u32> for PrintOptionStates {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintOptionStates {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintOptionStates {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.OptionDetails.PrintOptionStates;u4)");
}
impl ::windows::core::DefaultType for PrintOptionStates {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PrintOptionStates {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PrintOptionStates {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PrintOptionStates {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PrintOptionStates {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PrintOptionStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PrintOptionType(pub i32);
impl PrintOptionType {
    pub const Unknown: PrintOptionType = PrintOptionType(0i32);
    pub const Number: PrintOptionType = PrintOptionType(1i32);
    pub const Text: PrintOptionType = PrintOptionType(2i32);
    pub const ItemList: PrintOptionType = PrintOptionType(3i32);
    pub const Toggle: PrintOptionType = PrintOptionType(4i32);
}
impl ::core::convert::From<i32> for PrintOptionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PrintOptionType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PrintOptionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.OptionDetails.PrintOptionType;i4)");
}
impl ::windows::core::DefaultType for PrintOptionType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintOrientationOptionDetails(pub ::windows::core::IInspectable);
impl PrintOrientationOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintOrientationOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintOrientationOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintOrientationOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintOrientationOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintOrientationOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintOrientationOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintOrientationOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintOrientationOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintOrientationOptionDetails";
}
impl ::core::convert::From<PrintOrientationOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintOrientationOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintOrientationOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintOrientationOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintOrientationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintOrientationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintOrientationOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintOrientationOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintOrientationOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintOrientationOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintOrientationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintOrientationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintOrientationOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintOrientationOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintOrientationOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintOrientationOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintOrientationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintOrientationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintOrientationOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintOrientationOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintOrientationOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintOrientationOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for PrintOrientationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for &PrintOrientationOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::core::convert::TryInto::<IPrintItemListOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintOrientationOptionDetails {}
unsafe impl ::core::marker::Sync for PrintOrientationOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintPageRangeOptionDetails(pub ::windows::core::IInspectable);
impl PrintPageRangeOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintPageRangeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintPageRangeOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintPageRangeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintPageRangeOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintPageRangeOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintPageRangeOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintPageRangeOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintPageRangeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintPageRangeOptionDetails";
}
impl ::core::convert::From<PrintPageRangeOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintPageRangeOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintPageRangeOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintPageRangeOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintPageRangeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintPageRangeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintPageRangeOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintPageRangeOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintPageRangeOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintPageRangeOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintPageRangeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintPageRangeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintPageRangeOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintPageRangeOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintPageRangeOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintPageRangeOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintPageRangeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintPageRangeOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintPageRangeOptionDetails {}
unsafe impl ::core::marker::Sync for PrintPageRangeOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintQualityOptionDetails(pub ::windows::core::IInspectable);
impl PrintQualityOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintQualityOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintQualityOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintQualityOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintQualityOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintQualityOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintQualityOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintQualityOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintQualityOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintQualityOptionDetails";
}
impl ::core::convert::From<PrintQualityOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintQualityOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintQualityOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintQualityOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintQualityOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintQualityOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintQualityOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintQualityOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintQualityOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintQualityOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintQualityOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintQualityOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintQualityOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintQualityOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintQualityOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintQualityOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintQualityOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintQualityOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintQualityOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintQualityOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintQualityOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintQualityOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for PrintQualityOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for &PrintQualityOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::core::convert::TryInto::<IPrintItemListOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintQualityOptionDetails {}
unsafe impl ::core::marker::Sync for PrintQualityOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintStapleOptionDetails(pub ::windows::core::IInspectable);
impl PrintStapleOptionDetails {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetErrorText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__: PrintOptionStates = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintOptionStates>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn TrySetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetWarningText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintStapleOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintStapleOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintStapleOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintStapleOptionDetails>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintStapleOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintStapleOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
unsafe impl ::windows::core::Interface for PrintStapleOptionDetails {
    type Vtable = IPrintOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
impl ::windows::core::RuntimeName for PrintStapleOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintStapleOptionDetails";
}
impl ::core::convert::From<PrintStapleOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintStapleOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintStapleOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintStapleOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintStapleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintStapleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintStapleOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintStapleOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintStapleOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintStapleOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintStapleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintStapleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<PrintStapleOptionDetails> for IPrintOptionDetails {
    fn from(value: PrintStapleOptionDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintStapleOptionDetails> for IPrintOptionDetails {
    fn from(value: &PrintStapleOptionDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for PrintStapleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintOptionDetails> for &PrintStapleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintOptionDetails> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<PrintStapleOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintStapleOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintStapleOptionDetails> for IPrintItemListOptionDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintStapleOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for PrintStapleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPrintItemListOptionDetails> for &PrintStapleOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, IPrintItemListOptionDetails> {
        ::core::convert::TryInto::<IPrintItemListOptionDetails>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintStapleOptionDetails {}
unsafe impl ::core::marker::Sync for PrintStapleOptionDetails {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskOptionChangedEventArgs(pub ::windows::core::IInspectable);
impl PrintTaskOptionChangedEventArgs {
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskOptionChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintTaskOptionChangedEventArgs;{65197d05-a5ee-4307-9407-9acad147679c})");
}
unsafe impl ::windows::core::Interface for PrintTaskOptionChangedEventArgs {
    type Vtable = IPrintTaskOptionChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65197d05_a5ee_4307_9407_9acad147679c);
}
impl ::windows::core::RuntimeName for PrintTaskOptionChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintTaskOptionChangedEventArgs";
}
impl ::core::convert::From<PrintTaskOptionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintTaskOptionChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskOptionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskOptionChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskOptionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTaskOptionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskOptionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintTaskOptionChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskOptionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskOptionChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskOptionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTaskOptionChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintTaskOptionChangedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskOptionChangedEventArgs {}
#[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintTaskOptionDetails(pub ::windows::core::IInspectable);
impl PrintTaskOptionDetails {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn Options(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IPrintOptionDetails>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IPrintOptionDetails>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn CreateItemListOption<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, optionid: Param0, displayname: Param1) -> ::windows::core::Result<PrintCustomItemListOptionDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), optionid.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<PrintCustomItemListOptionDetails>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn CreateTextOption<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, optionid: Param0, displayname: Param1) -> ::windows::core::Result<PrintCustomTextOptionDetails> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), optionid.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<PrintCustomTextOptionDetails>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation`*"]
    pub fn OptionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, PrintTaskOptionChangedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation`*"]
    pub fn RemoveOptionChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation`*"]
    pub fn BeginValidation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, ::windows::core::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation`*"]
    pub fn RemoveBeginValidation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation`*"]
    pub fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows::core::Result<super::PrintPageDescription> {
        let this = &::windows::core::Interface::cast::<super::IPrintTaskOptionsCore>(self)?;
        unsafe {
            let mut result__: super::PrintPageDescription = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), jobpagenumber, &mut result__).from_abi::<super::PrintPageDescription>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`, `Foundation_Collections`*"]
    pub fn DisplayedOptions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<super::IPrintTaskOptionsCoreUIConfiguration>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn GetFromPrintTaskOptions<'a, Param0: ::windows::core::IntoParam<'a, super::PrintTaskOptions>>(printtaskoptions: Param0) -> ::windows::core::Result<PrintTaskOptionDetails> {
        Self::IPrintTaskOptionDetailsStatic(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), printtaskoptions.into_param().abi(), &mut result__).from_abi::<PrintTaskOptionDetails>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing_OptionDetails`*"]
    pub fn CreateToggleOption<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, optionid: Param0, displayname: Param1) -> ::windows::core::Result<PrintCustomToggleOptionDetails> {
        let this = &::windows::core::Interface::cast::<IPrintTaskOptionDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), optionid.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<PrintCustomToggleOptionDetails>(result__)
        }
    }
    pub fn IPrintTaskOptionDetailsStatic<R, F: FnOnce(&IPrintTaskOptionDetailsStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PrintTaskOptionDetails, IPrintTaskOptionDetailsStatic> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTaskOptionDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails;{f5720af1-a89e-42a6-81af-f8e010b38a68})");
}
unsafe impl ::windows::core::Interface for PrintTaskOptionDetails {
    type Vtable = IPrintTaskOptionDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5720af1_a89e_42a6_81af_f8e010b38a68);
}
impl ::windows::core::RuntimeName for PrintTaskOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails";
}
impl ::core::convert::From<PrintTaskOptionDetails> for ::windows::core::IUnknown {
    fn from(value: PrintTaskOptionDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintTaskOptionDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintTaskOptionDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTaskOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTaskOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintTaskOptionDetails> for ::windows::core::IInspectable {
    fn from(value: PrintTaskOptionDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintTaskOptionDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintTaskOptionDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTaskOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTaskOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<PrintTaskOptionDetails> for super::IPrintTaskOptionsCore {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintTaskOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskOptionDetails> for super::IPrintTaskOptionsCore {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IPrintTaskOptionsCore> for PrintTaskOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, super::IPrintTaskOptionsCore> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IPrintTaskOptionsCore> for &PrintTaskOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, super::IPrintTaskOptionsCore> {
        ::core::convert::TryInto::<super::IPrintTaskOptionsCore>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<PrintTaskOptionDetails> for super::IPrintTaskOptionsCoreUIConfiguration {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintTaskOptionDetails) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&PrintTaskOptionDetails> for super::IPrintTaskOptionsCoreUIConfiguration {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintTaskOptionDetails) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IPrintTaskOptionsCoreUIConfiguration> for PrintTaskOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, super::IPrintTaskOptionsCoreUIConfiguration> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IPrintTaskOptionsCoreUIConfiguration> for &PrintTaskOptionDetails {
    fn into_param(self) -> ::windows::core::Param<'a, super::IPrintTaskOptionsCoreUIConfiguration> {
        ::core::convert::TryInto::<super::IPrintTaskOptionsCoreUIConfiguration>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintTaskOptionDetails {}
unsafe impl ::core::marker::Sync for PrintTaskOptionDetails {}
