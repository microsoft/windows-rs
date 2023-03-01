#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintBindingOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintBindingOptionDetails {
    type Vtable = IPrintBindingOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintBindingOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintBindingOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3f4cc98_9564_4f16_a055_a98b9a49e9d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintBindingOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintBorderingOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintBorderingOptionDetails {
    type Vtable = IPrintBorderingOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintBorderingOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintBorderingOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d73bc8f_fb53_4eb2_985f_1d91de0b7639);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintBorderingOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintCollationOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintCollationOptionDetails {
    type Vtable = IPrintCollationOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintCollationOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintCollationOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6abb166_a5a6_40dc_acc3_739f28f1e5d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCollationOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintColorModeOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintColorModeOptionDetails {
    type Vtable = IPrintColorModeOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintColorModeOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintColorModeOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdba97704_f1d6_4843_a484_9b447cdcf3b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintColorModeOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintCopiesOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintCopiesOptionDetails {
    type Vtable = IPrintCopiesOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintCopiesOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintCopiesOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42053099_4339_4343_898d_2c47b5e0c341);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCopiesOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintCustomItemDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintCustomItemDetails {
    type Vtable = IPrintCustomItemDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintCustomItemDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintCustomItemDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5704b637_5c3a_449a_aa36_b3291b1192fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomItemDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetItemDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ItemDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintCustomItemListOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintCustomItemListOptionDetails {
    type Vtable = IPrintCustomItemListOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintCustomItemListOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintCustomItemListOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5fafd88_58f2_4ebd_b90f_51e4f2944c5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomItemListOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintCustomItemListOptionDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintCustomItemListOptionDetails2 {
    type Vtable = IPrintCustomItemListOptionDetails2_Vtbl;
}
impl ::core::clone::Clone for IPrintCustomItemListOptionDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintCustomItemListOptionDetails2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9d6353d_651c_4a39_906e_1091a1801bf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomItemListOptionDetails2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, description: ::std::mem::MaybeUninit<::windows::core::HSTRING>, icon: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AddItem: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintCustomItemListOptionDetails3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintCustomItemListOptionDetails3 {
    type Vtable = IPrintCustomItemListOptionDetails3_Vtbl;
}
impl ::core::clone::Clone for IPrintCustomItemListOptionDetails3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintCustomItemListOptionDetails3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fa1b53f_3c34_4868_a407_fc5eab259b21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomItemListOptionDetails3_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct IPrintCustomOptionDetails(::windows::core::IUnknown);
impl IPrintCustomOptionDetails {
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IPrintCustomOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IPrintOptionDetails> for IPrintCustomOptionDetails {}
impl ::core::cmp::PartialEq for IPrintCustomOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintCustomOptionDetails {}
impl ::core::fmt::Debug for IPrintCustomOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintCustomOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IPrintCustomOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{e32bde1c-28af-4b90-95da-a3acf320b929}");
}
unsafe impl ::windows::core::Interface for IPrintCustomOptionDetails {
    type Vtable = IPrintCustomOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintCustomOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintCustomOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe32bde1c_28af_4b90_95da_a3acf320b929);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintCustomTextOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintCustomTextOptionDetails {
    type Vtable = IPrintCustomTextOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintCustomTextOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintCustomTextOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ad171f8_c8bd_4905_9192_0d75136e8b31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomTextOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetMaxCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub MaxCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintCustomTextOptionDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintCustomTextOptionDetails2 {
    type Vtable = IPrintCustomTextOptionDetails2_Vtbl;
}
impl ::core::clone::Clone for IPrintCustomTextOptionDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintCustomTextOptionDetails2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcea70b54_b977_4718_8338_7ed2b0d86fe3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomTextOptionDetails2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintCustomToggleOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintCustomToggleOptionDetails {
    type Vtable = IPrintCustomToggleOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintCustomToggleOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintCustomToggleOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9db4d514_e461_4608_8ee9_db6f5ed073c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintCustomToggleOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintDuplexOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintDuplexOptionDetails {
    type Vtable = IPrintDuplexOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintDuplexOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintDuplexOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfcd94591_d4a4_44fa_b3fe_42e0ba28d5ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDuplexOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintHolePunchOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintHolePunchOptionDetails {
    type Vtable = IPrintHolePunchOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintHolePunchOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintHolePunchOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6de1f18_482c_4657_9d71_8ddddbea1e1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintHolePunchOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct IPrintItemListOptionDetails(::windows::core::IUnknown);
impl IPrintItemListOptionDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IPrintItemListOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IPrintOptionDetails> for IPrintItemListOptionDetails {}
impl ::core::cmp::PartialEq for IPrintItemListOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintItemListOptionDetails {}
impl ::core::fmt::Debug for IPrintItemListOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintItemListOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IPrintItemListOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{9a2257bf-fe61-43d8-a24f-a3f6ab7320e7}");
}
unsafe impl ::windows::core::Interface for IPrintItemListOptionDetails {
    type Vtable = IPrintItemListOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintItemListOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintItemListOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a2257bf_fe61_43d8_a24f_a3f6ab7320e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintItemListOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Items: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Items: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintMediaSizeOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintMediaSizeOptionDetails {
    type Vtable = IPrintMediaSizeOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintMediaSizeOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintMediaSizeOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c8d5bcf_c0bf_47c8_b84a_628e7d0d1a1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintMediaSizeOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintMediaTypeOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintMediaTypeOptionDetails {
    type Vtable = IPrintMediaTypeOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintMediaTypeOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintMediaTypeOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8c7000b_abf3_4abc_8e86_22abc5744a43);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintMediaTypeOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct IPrintNumberOptionDetails(::windows::core::IUnknown);
impl IPrintNumberOptionDetails {
    pub fn MinValue(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxValue(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IPrintNumberOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IPrintOptionDetails> for IPrintNumberOptionDetails {}
impl ::core::cmp::PartialEq for IPrintNumberOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintNumberOptionDetails {}
impl ::core::fmt::Debug for IPrintNumberOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintNumberOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IPrintNumberOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{4d01bbaf-645c-4de9-965f-6fc6bbc47cab}");
}
unsafe impl ::windows::core::Interface for IPrintNumberOptionDetails {
    type Vtable = IPrintNumberOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintNumberOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintNumberOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d01bbaf_645c_4de9_965f_6fc6bbc47cab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintNumberOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MinValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct IPrintOptionDetails(::windows::core::IUnknown);
impl IPrintOptionDetails {
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IPrintOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IPrintOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintOptionDetails {}
impl ::core::fmt::Debug for IPrintOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IPrintOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{390686cf-d682-495f-adfe-d7333f5c1808}");
}
unsafe impl ::windows::core::Interface for IPrintOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x390686cf_d682_495f_adfe_d7333f5c1808);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OptionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub OptionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintOptionType) -> ::windows::core::HRESULT,
    pub SetErrorText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ErrorText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintOptionStates) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintOptionStates) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TrySetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintOrientationOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintOrientationOptionDetails {
    type Vtable = IPrintOrientationOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintOrientationOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintOrientationOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46c38879_66e0_4da0_87b4_d25457824eb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintOrientationOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintPageRangeOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintPageRangeOptionDetails {
    type Vtable = IPrintPageRangeOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintPageRangeOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintPageRangeOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a19e4b7_2be8_4aa7_9ea5_defbe8713b4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintPageRangeOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintQualityOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintQualityOptionDetails {
    type Vtable = IPrintQualityOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintQualityOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintQualityOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2dd06ba1_ce1a_44e6_84f9_3a92ea1e3044);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintQualityOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintStapleOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintStapleOptionDetails {
    type Vtable = IPrintStapleOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintStapleOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintStapleOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd43175bd_9c0b_44e0_84f6_ceebce653800);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintStapleOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetWarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub WarningText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskOptionChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskOptionChangedEventArgs {
    type Vtable = IPrintTaskOptionChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintTaskOptionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTaskOptionChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65197d05_a5ee_4307_9407_9acad147679c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub OptionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskOptionDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskOptionDetails {
    type Vtable = IPrintTaskOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintTaskOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTaskOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5720af1_a89e_42a6_81af_f8e010b38a68);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Options: usize,
    pub CreateItemListOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTextOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OptionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OptionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOptionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOptionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub BeginValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeginValidation: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBeginValidation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBeginValidation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskOptionDetails2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskOptionDetails2 {
    type Vtable = IPrintTaskOptionDetails2_Vtbl;
}
impl ::core::clone::Clone for IPrintTaskOptionDetails2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTaskOptionDetails2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53730a09_f968_4692_a177_c074597186db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionDetails2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateToggleOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, displayname: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTaskOptionDetailsStatic(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTaskOptionDetailsStatic {
    type Vtable = IPrintTaskOptionDetailsStatic_Vtbl;
}
impl ::core::clone::Clone for IPrintTaskOptionDetailsStatic {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTaskOptionDetailsStatic {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x135da193_0961_4b6e_8766_f13b7fbccd58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTaskOptionDetailsStatic_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetFromPrintTaskOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printtaskoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct IPrintTextOptionDetails(::windows::core::IUnknown);
impl IPrintTextOptionDetails {
    pub fn MaxCharacters(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxCharacters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = &::windows::core::ComInterface::cast::<IPrintOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IPrintTextOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl windows::core::CanTryInto<IPrintOptionDetails> for IPrintTextOptionDetails {}
impl ::core::cmp::PartialEq for IPrintTextOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintTextOptionDetails {}
impl ::core::fmt::Debug for IPrintTextOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintTextOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IPrintTextOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{ad75e563-5ce4-46bc-9918-ab9fad144c5b}");
}
unsafe impl ::windows::core::Interface for IPrintTextOptionDetails {
    type Vtable = IPrintTextOptionDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintTextOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTextOptionDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad75e563_5ce4_46bc_9918_ab9fad144c5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTextOptionDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MaxCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintBindingOptionDetails(::windows::core::IUnknown);
impl PrintBindingOptionDetails {
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintBindingOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintBindingOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintBindingOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintBindingOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintBindingOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintBindingOptionDetails {}
impl ::core::fmt::Debug for PrintBindingOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintBindingOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintBindingOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintBindingOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintBindingOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintBindingOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintBindingOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintBindingOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintBindingOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintBindingOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintItemListOptionDetails> for PrintBindingOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintBindingOptionDetails {}
unsafe impl ::core::marker::Send for PrintBindingOptionDetails {}
unsafe impl ::core::marker::Sync for PrintBindingOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintBorderingOptionDetails(::windows::core::IUnknown);
impl PrintBorderingOptionDetails {
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintBorderingOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintBorderingOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintBorderingOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintBorderingOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintBorderingOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintBorderingOptionDetails {}
impl ::core::fmt::Debug for PrintBorderingOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintBorderingOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintBorderingOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintBorderingOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintBorderingOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintBorderingOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintBorderingOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintBorderingOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintBorderingOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintBorderingOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintItemListOptionDetails> for PrintBorderingOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintBorderingOptionDetails {}
unsafe impl ::core::marker::Send for PrintBorderingOptionDetails {}
unsafe impl ::core::marker::Sync for PrintBorderingOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintCollationOptionDetails(::windows::core::IUnknown);
impl PrintCollationOptionDetails {
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCollationOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCollationOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCollationOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCollationOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintCollationOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintCollationOptionDetails {}
impl ::core::fmt::Debug for PrintCollationOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintCollationOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintCollationOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintCollationOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintCollationOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintCollationOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintCollationOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintCollationOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCollationOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintCollationOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintItemListOptionDetails> for PrintCollationOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintCollationOptionDetails {}
unsafe impl ::core::marker::Send for PrintCollationOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCollationOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintColorModeOptionDetails(::windows::core::IUnknown);
impl PrintColorModeOptionDetails {
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintColorModeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintColorModeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintColorModeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintColorModeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintColorModeOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintColorModeOptionDetails {}
impl ::core::fmt::Debug for PrintColorModeOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintColorModeOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintColorModeOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintColorModeOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintColorModeOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintColorModeOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintColorModeOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintColorModeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintColorModeOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintColorModeOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintItemListOptionDetails> for PrintColorModeOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintColorModeOptionDetails {}
unsafe impl ::core::marker::Send for PrintColorModeOptionDetails {}
unsafe impl ::core::marker::Sync for PrintColorModeOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintCopiesOptionDetails(::windows::core::IUnknown);
impl PrintCopiesOptionDetails {
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCopiesOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCopiesOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCopiesOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCopiesOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MinValue(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IPrintNumberOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MinValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxValue(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IPrintNumberOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxValue)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintCopiesOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintCopiesOptionDetails {}
impl ::core::fmt::Debug for PrintCopiesOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintCopiesOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintCopiesOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintCopiesOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintCopiesOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintCopiesOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintCopiesOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintCopiesOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCopiesOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintCopiesOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintNumberOptionDetails> for PrintCopiesOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintCopiesOptionDetails {}
unsafe impl ::core::marker::Send for PrintCopiesOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCopiesOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintCustomItemDetails(::windows::core::IUnknown);
impl PrintCustomItemDetails {
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ItemId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetItemDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetItemDisplayName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ItemDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ItemDisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintCustomItemDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintCustomItemDetails {}
impl ::core::fmt::Debug for PrintCustomItemDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintCustomItemDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintCustomItemDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintCustomItemDetails;{5704b637-5c3a-449a-aa36-b3291b1192fd})");
}
impl ::core::clone::Clone for PrintCustomItemDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintCustomItemDetails {
    type Vtable = IPrintCustomItemDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintCustomItemDetails {
    const IID: ::windows::core::GUID = <IPrintCustomItemDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintCustomItemDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCustomItemDetails";
}
::windows::imp::interface_hierarchy!(PrintCustomItemDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintCustomItemDetails {}
unsafe impl ::core::marker::Sync for PrintCustomItemDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintCustomItemListOptionDetails(::windows::core::IUnknown);
impl PrintCustomItemListOptionDetails {
    pub fn AddItem(&self, itemid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomItemListOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddItem)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(itemid), ::core::mem::transmute_copy(displayname)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AddItem2<P0>(&self, itemid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING, description: &::windows::core::HSTRING, icon: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Storage::Streams::IRandomAccessStreamWithContentType>,
    {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomItemListOptionDetails2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddItem)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(itemid), ::core::mem::transmute_copy(displayname), ::core::mem::transmute_copy(description), icon.try_into_param()?.abi()).ok() }
    }
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomItemListOptionDetails3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomItemListOptionDetails3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomItemListOptionDetails3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomItemListOptionDetails3>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintCustomItemListOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintCustomItemListOptionDetails {}
impl ::core::fmt::Debug for PrintCustomItemListOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintCustomItemListOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintCustomItemListOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintCustomItemListOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintCustomItemListOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintCustomItemListOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintCustomItemListOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintCustomItemListOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintCustomOptionDetails> for PrintCustomItemListOptionDetails {}
impl ::windows::core::CanTryInto<IPrintItemListOptionDetails> for PrintCustomItemListOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintCustomItemListOptionDetails {}
unsafe impl ::core::marker::Send for PrintCustomItemListOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCustomItemListOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintCustomTextOptionDetails(::windows::core::IUnknown);
impl PrintCustomTextOptionDetails {
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxCharacters(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomTextOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxCharacters)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxCharacters(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomTextOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaxCharacters)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomTextOptionDetails2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomTextOptionDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomTextOptionDetails2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomTextOptionDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintCustomTextOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintCustomTextOptionDetails {}
impl ::core::fmt::Debug for PrintCustomTextOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintCustomTextOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintCustomTextOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintCustomTextOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintCustomTextOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintCustomTextOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintCustomTextOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintCustomTextOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintCustomOptionDetails> for PrintCustomTextOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintCustomTextOptionDetails {}
unsafe impl ::core::marker::Send for PrintCustomTextOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCustomTextOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintCustomToggleOptionDetails(::windows::core::IUnknown);
impl PrintCustomToggleOptionDetails {
    pub fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomToggleOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomToggleOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomToggleOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintCustomToggleOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintCustomToggleOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintCustomToggleOptionDetails {}
impl ::core::fmt::Debug for PrintCustomToggleOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintCustomToggleOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintCustomToggleOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintCustomToggleOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintCustomToggleOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintCustomToggleOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintCustomToggleOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintCustomToggleOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintCustomToggleOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintCustomToggleOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintCustomOptionDetails> for PrintCustomToggleOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintCustomToggleOptionDetails {}
unsafe impl ::core::marker::Send for PrintCustomToggleOptionDetails {}
unsafe impl ::core::marker::Sync for PrintCustomToggleOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintDuplexOptionDetails(::windows::core::IUnknown);
impl PrintDuplexOptionDetails {
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintDuplexOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintDuplexOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintDuplexOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintDuplexOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintDuplexOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintDuplexOptionDetails {}
impl ::core::fmt::Debug for PrintDuplexOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintDuplexOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintDuplexOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintDuplexOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintDuplexOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintDuplexOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintDuplexOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintDuplexOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintDuplexOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintDuplexOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintItemListOptionDetails> for PrintDuplexOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintDuplexOptionDetails {}
unsafe impl ::core::marker::Send for PrintDuplexOptionDetails {}
unsafe impl ::core::marker::Sync for PrintDuplexOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintHolePunchOptionDetails(::windows::core::IUnknown);
impl PrintHolePunchOptionDetails {
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintHolePunchOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintHolePunchOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintHolePunchOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintHolePunchOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintHolePunchOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintHolePunchOptionDetails {}
impl ::core::fmt::Debug for PrintHolePunchOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintHolePunchOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintHolePunchOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintHolePunchOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintHolePunchOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintHolePunchOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintHolePunchOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintHolePunchOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintHolePunchOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintHolePunchOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintItemListOptionDetails> for PrintHolePunchOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintHolePunchOptionDetails {}
unsafe impl ::core::marker::Send for PrintHolePunchOptionDetails {}
unsafe impl ::core::marker::Sync for PrintHolePunchOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintMediaSizeOptionDetails(::windows::core::IUnknown);
impl PrintMediaSizeOptionDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintMediaSizeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintMediaSizeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintMediaSizeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintMediaSizeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintMediaSizeOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintMediaSizeOptionDetails {}
impl ::core::fmt::Debug for PrintMediaSizeOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintMediaSizeOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintMediaSizeOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintMediaSizeOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintMediaSizeOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintMediaSizeOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintMediaSizeOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintMediaSizeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintMediaSizeOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintMediaSizeOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintItemListOptionDetails> for PrintMediaSizeOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintMediaSizeOptionDetails {}
unsafe impl ::core::marker::Send for PrintMediaSizeOptionDetails {}
unsafe impl ::core::marker::Sync for PrintMediaSizeOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintMediaTypeOptionDetails(::windows::core::IUnknown);
impl PrintMediaTypeOptionDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintMediaTypeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintMediaTypeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintMediaTypeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintMediaTypeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintMediaTypeOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintMediaTypeOptionDetails {}
impl ::core::fmt::Debug for PrintMediaTypeOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintMediaTypeOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintMediaTypeOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintMediaTypeOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintMediaTypeOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintMediaTypeOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintMediaTypeOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintMediaTypeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintMediaTypeOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintMediaTypeOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintItemListOptionDetails> for PrintMediaTypeOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintMediaTypeOptionDetails {}
unsafe impl ::core::marker::Send for PrintMediaTypeOptionDetails {}
unsafe impl ::core::marker::Sync for PrintMediaTypeOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintOrientationOptionDetails(::windows::core::IUnknown);
impl PrintOrientationOptionDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintOrientationOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintOrientationOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintOrientationOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintOrientationOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintOrientationOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintOrientationOptionDetails {}
impl ::core::fmt::Debug for PrintOrientationOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintOrientationOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintOrientationOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintOrientationOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintOrientationOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintOrientationOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintOrientationOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintOrientationOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintOrientationOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintOrientationOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintItemListOptionDetails> for PrintOrientationOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintOrientationOptionDetails {}
unsafe impl ::core::marker::Send for PrintOrientationOptionDetails {}
unsafe impl ::core::marker::Sync for PrintOrientationOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintPageRangeOptionDetails(::windows::core::IUnknown);
impl PrintPageRangeOptionDetails {
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintPageRangeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintPageRangeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintPageRangeOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintPageRangeOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintPageRangeOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintPageRangeOptionDetails {}
impl ::core::fmt::Debug for PrintPageRangeOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintPageRangeOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintPageRangeOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintPageRangeOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintPageRangeOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintPageRangeOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintPageRangeOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintPageRangeOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintPageRangeOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintPageRangeOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintPageRangeOptionDetails {}
unsafe impl ::core::marker::Send for PrintPageRangeOptionDetails {}
unsafe impl ::core::marker::Sync for PrintPageRangeOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintQualityOptionDetails(::windows::core::IUnknown);
impl PrintQualityOptionDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintQualityOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintQualityOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintQualityOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintQualityOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintQualityOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintQualityOptionDetails {}
impl ::core::fmt::Debug for PrintQualityOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintQualityOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintQualityOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintQualityOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintQualityOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintQualityOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintQualityOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintQualityOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintQualityOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintQualityOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintItemListOptionDetails> for PrintQualityOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintQualityOptionDetails {}
unsafe impl ::core::marker::Send for PrintQualityOptionDetails {}
unsafe impl ::core::marker::Sync for PrintQualityOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintStapleOptionDetails(::windows::core::IUnknown);
impl PrintStapleOptionDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::ComInterface::cast::<IPrintItemListOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>();
            (::windows::core::Interface::vtable(this).Items)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OptionType(&self) -> ::windows::core::Result<PrintOptionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionType>();
            (::windows::core::Interface::vtable(this).OptionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetErrorText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ErrorText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ErrorText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetState(&self, value: PrintOptionStates) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetState)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn State(&self) -> ::windows::core::Result<PrintOptionStates> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintOptionStates>();
            (::windows::core::Interface::vtable(this).State)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrySetValue<P0>(&self, value: P0) -> ::windows::core::Result<bool>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).TrySetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWarningText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintStapleOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWarningText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn WarningText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintStapleOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).WarningText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintStapleOptionDetails>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintStapleOptionDetails>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintStapleOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintStapleOptionDetails {}
impl ::core::fmt::Debug for PrintStapleOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintStapleOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintStapleOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintStapleOptionDetails;{390686cf-d682-495f-adfe-d7333f5c1808})");
}
impl ::core::clone::Clone for PrintStapleOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintStapleOptionDetails {
    type Vtable = IPrintOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintStapleOptionDetails {
    const IID: ::windows::core::GUID = <IPrintOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintStapleOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintStapleOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintStapleOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IPrintItemListOptionDetails> for PrintStapleOptionDetails {}
impl ::windows::core::CanTryInto<IPrintOptionDetails> for PrintStapleOptionDetails {}
unsafe impl ::core::marker::Send for PrintStapleOptionDetails {}
unsafe impl ::core::marker::Sync for PrintStapleOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintTaskOptionChangedEventArgs(::windows::core::IUnknown);
impl PrintTaskOptionChangedEventArgs {
    pub fn OptionId(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).OptionId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintTaskOptionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskOptionChangedEventArgs {}
impl ::core::fmt::Debug for PrintTaskOptionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskOptionChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintTaskOptionChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintTaskOptionChangedEventArgs;{65197d05-a5ee-4307-9407-9acad147679c})");
}
impl ::core::clone::Clone for PrintTaskOptionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintTaskOptionChangedEventArgs {
    type Vtable = IPrintTaskOptionChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintTaskOptionChangedEventArgs {
    const IID: ::windows::core::GUID = <IPrintTaskOptionChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskOptionChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintTaskOptionChangedEventArgs";
}
::windows::imp::interface_hierarchy!(PrintTaskOptionChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintTaskOptionChangedEventArgs {}
unsafe impl ::core::marker::Sync for PrintTaskOptionChangedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
pub struct PrintTaskOptionDetails(::windows::core::IUnknown);
impl PrintTaskOptionDetails {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Options(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IPrintOptionDetails>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IPrintOptionDetails>>();
            (::windows::core::Interface::vtable(this).Options)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateItemListOption(&self, optionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PrintCustomItemListOptionDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintCustomItemListOptionDetails>();
            (::windows::core::Interface::vtable(this).CreateItemListOption)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(optionid), ::core::mem::transmute_copy(displayname), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateTextOption(&self, optionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PrintCustomTextOptionDetails> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintCustomTextOptionDetails>();
            (::windows::core::Interface::vtable(this).CreateTextOption)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(optionid), ::core::mem::transmute_copy(displayname), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OptionChanged(&self, eventhandler: &super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, PrintTaskOptionChangedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).OptionChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionChanged(&self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOptionChanged)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeginValidation(&self, eventhandler: &super::super::super::Foundation::TypedEventHandler<PrintTaskOptionDetails, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).BeginValidation)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBeginValidation(&self, eventcookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBeginValidation)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn CreateToggleOption(&self, optionid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PrintCustomToggleOptionDetails> {
        let this = &::windows::core::ComInterface::cast::<IPrintTaskOptionDetails2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintCustomToggleOptionDetails>();
            (::windows::core::Interface::vtable(this).CreateToggleOption)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(optionid), ::core::mem::transmute_copy(displayname), &mut result__).from_abi(result__)
        }
    }
    pub fn GetFromPrintTaskOptions(printtaskoptions: &super::PrintTaskOptions) -> ::windows::core::Result<PrintTaskOptionDetails> {
        Self::IPrintTaskOptionDetailsStatic(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTaskOptionDetails>();
            (::windows::core::Interface::vtable(this).GetFromPrintTaskOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(printtaskoptions), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows::core::Result<super::PrintPageDescription> {
        let this = &::windows::core::ComInterface::cast::<super::IPrintTaskOptionsCore>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::PrintPageDescription>();
            (::windows::core::Interface::vtable(this).GetPageDescription)(::windows::core::Interface::as_raw(this), jobpagenumber, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DisplayedOptions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::ComInterface::cast::<super::IPrintTaskOptionsCoreUIConfiguration>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).DisplayedOptions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IPrintTaskOptionDetailsStatic<R, F: FnOnce(&IPrintTaskOptionDetailsStatic) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PrintTaskOptionDetails, IPrintTaskOptionDetailsStatic> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PrintTaskOptionDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTaskOptionDetails {}
impl ::core::fmt::Debug for PrintTaskOptionDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTaskOptionDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintTaskOptionDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails;{f5720af1-a89e-42a6-81af-f8e010b38a68})");
}
impl ::core::clone::Clone for PrintTaskOptionDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintTaskOptionDetails {
    type Vtable = IPrintTaskOptionDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintTaskOptionDetails {
    const IID: ::windows::core::GUID = <IPrintTaskOptionDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintTaskOptionDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails";
}
::windows::imp::interface_hierarchy!(PrintTaskOptionDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<super::IPrintTaskOptionsCore> for PrintTaskOptionDetails {}
impl ::windows::core::CanTryInto<super::IPrintTaskOptionsCoreUIConfiguration> for PrintTaskOptionDetails {}
unsafe impl ::core::marker::Send for PrintTaskOptionDetails {}
unsafe impl ::core::marker::Sync for PrintTaskOptionDetails {}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintOptionStates(pub u32);
impl PrintOptionStates {
    pub const None: Self = Self(0u32);
    pub const Enabled: Self = Self(1u32);
    pub const Constrained: Self = Self(2u32);
}
impl ::core::marker::Copy for PrintOptionStates {}
impl ::core::clone::Clone for PrintOptionStates {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintOptionStates {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PrintOptionStates {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PrintOptionStates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintOptionStates").field(&self.0).finish()
    }
}
impl PrintOptionStates {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PrintOptionStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PrintOptionStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PrintOptionStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PrintOptionStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PrintOptionStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows::core::RuntimeType for PrintOptionStates {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.OptionDetails.PrintOptionStates;u4)");
}
#[doc = "*Required features: `\"Graphics_Printing_OptionDetails\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintOptionType(pub i32);
impl PrintOptionType {
    pub const Unknown: Self = Self(0i32);
    pub const Number: Self = Self(1i32);
    pub const Text: Self = Self(2i32);
    pub const ItemList: Self = Self(3i32);
    pub const Toggle: Self = Self(4i32);
}
impl ::core::marker::Copy for PrintOptionType {}
impl ::core::clone::Clone for PrintOptionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintOptionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PrintOptionType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PrintOptionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintOptionType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintOptionType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.OptionDetails.PrintOptionType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
