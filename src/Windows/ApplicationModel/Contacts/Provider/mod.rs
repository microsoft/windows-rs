#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AddContactResult(pub i32);
impl AddContactResult {
    pub const Added: AddContactResult = AddContactResult(0i32);
    pub const AlreadyAdded: AddContactResult = AddContactResult(1i32);
    pub const Unavailable: AddContactResult = AddContactResult(2i32);
}
impl ::core::convert::From<i32> for AddContactResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AddContactResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AddContactResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.Provider.AddContactResult;i4)");
}
impl ::windows::core::DefaultType for AddContactResult {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactPickerUI(pub ::windows::core::IInspectable);
impl ContactPickerUI {
    #[cfg(feature = "deprecated")]
    pub fn AddContact<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Contact>>(&self, id: Param0, contact: Param1) -> ::windows::core::Result<AddContactResult> {
        let this = self;
        unsafe {
            let mut result__: AddContactResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), id.into_param().abi(), contact.into_param().abi(), &mut result__).from_abi::<AddContactResult>(result__)
        }
    }
    pub fn RemoveContact<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, id: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id.into_param().abi()).ok() }
    }
    pub fn ContainsContact<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, id: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFields(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn SelectionMode(&self) -> ::windows::core::Result<super::ContactSelectionMode> {
        let this = self;
        unsafe {
            let mut result__: super::ContactSelectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::ContactSelectionMode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ContactRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<ContactPickerUI, ContactRemovedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveContactRemoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn AddContact2<'a, Param0: ::windows::core::IntoParam<'a, super::Contact>>(&self, contact: Param0) -> ::windows::core::Result<AddContactResult> {
        let this = &::windows::core::Interface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__: AddContactResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), contact.into_param().abi(), &mut result__).from_abi::<AddContactResult>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFieldsWithContactFieldType(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::ContactFieldType>> {
        let this = &::windows::core::Interface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<super::ContactFieldType>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactPickerUI {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Provider.ContactPickerUI;{e2cc1366-cf66-43c4-a96a-a5a112db4746})");
}
unsafe impl ::windows::core::Interface for ContactPickerUI {
    type Vtable = IContactPickerUI_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2cc1366_cf66_43c4_a96a_a5a112db4746);
}
impl ::windows::core::RuntimeName for ContactPickerUI {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactPickerUI";
}
impl ::core::convert::From<ContactPickerUI> for ::windows::core::IUnknown {
    fn from(value: ContactPickerUI) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactPickerUI> for ::windows::core::IUnknown {
    fn from(value: &ContactPickerUI) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactPickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactPickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactPickerUI> for ::windows::core::IInspectable {
    fn from(value: ContactPickerUI) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactPickerUI> for ::windows::core::IInspectable {
    fn from(value: &ContactPickerUI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactPickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactPickerUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ContactRemovedEventArgs(pub ::windows::core::IInspectable);
impl ContactRemovedEventArgs {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ContactRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs;{6f354338-3302-4d13-ad8d-adcc0ff9e47c})");
}
unsafe impl ::windows::core::Interface for ContactRemovedEventArgs {
    type Vtable = IContactRemovedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f354338_3302_4d13_ad8d_adcc0ff9e47c);
}
impl ::windows::core::RuntimeName for ContactRemovedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs";
}
impl ::core::convert::From<ContactRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactRemovedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ContactRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactRemovedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContactRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContactRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ContactRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ContactRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContactRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContactRemovedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactPickerUI(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactPickerUI {
    type Vtable = IContactPickerUI_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2cc1366_cf66_43c4_a96a_a5a112db4746);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerUI_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contact: ::windows::core::RawPtr, result__: *mut AddContactResult) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::ContactSelectionMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactPickerUI2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactPickerUI2 {
    type Vtable = IContactPickerUI2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e449e28_7b25_4999_9b0b_875400a1e8c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerUI2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, contact: ::windows::core::RawPtr, result__: *mut AddContactResult) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IContactRemovedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IContactRemovedEventArgs {
    type Vtable = IContactRemovedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f354338_3302_4d13_ad8d_adcc0ff9e47c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactRemovedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
