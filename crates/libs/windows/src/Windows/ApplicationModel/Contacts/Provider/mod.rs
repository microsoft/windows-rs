#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPickerUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactPickerUI {
    type Vtable = IContactPickerUI_Vtbl;
}
impl ::core::clone::Clone for IContactPickerUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactPickerUI {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2cc1366_cf66_43c4_a96a_a5a112db4746);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerUI_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "deprecated")]
    pub AddContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, contact: *mut ::core::ffi::c_void, result__: *mut AddContactResult) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AddContact: usize,
    pub RemoveContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContainsContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub DesiredFields: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "deprecated")))]
    DesiredFields: usize,
    pub SelectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ContactSelectionMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContactRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContactRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContactRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContactRemoved: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPickerUI2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactPickerUI2 {
    type Vtable = IContactPickerUI2_Vtbl;
}
impl ::core::clone::Clone for IContactPickerUI2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactPickerUI2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e449e28_7b25_4999_9b0b_875400a1e8c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerUI2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AddContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contact: *mut ::core::ffi::c_void, result__: *mut AddContactResult) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DesiredFieldsWithContactFieldType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DesiredFieldsWithContactFieldType: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactRemovedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactRemovedEventArgs {
    type Vtable = IContactRemovedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IContactRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContactRemovedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f354338_3302_4d13_ad8d_adcc0ff9e47c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactRemovedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_Contacts_Provider\"`*"]
#[repr(transparent)]
pub struct ContactPickerUI(::windows::core::IUnknown);
impl ContactPickerUI {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AddContact(&self, id: &::windows::core::HSTRING, contact: &super::Contact) -> ::windows::core::Result<AddContactResult> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AddContactResult>();
            (::windows::core::Interface::vtable(this).AddContact)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(contact), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveContact(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContact)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id)).ok() }
    }
    pub fn ContainsContact(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).ContainsContact)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn DesiredFields(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).DesiredFields)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SelectionMode(&self) -> ::windows::core::Result<super::ContactSelectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::ContactSelectionMode>();
            (::windows::core::Interface::vtable(this).SelectionMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContactRemoved(&self, handler: &super::super::super::Foundation::TypedEventHandler<ContactPickerUI, ContactRemovedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ContactRemoved)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContactRemoved(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContactRemoved)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn AddContact2(&self, contact: &super::Contact) -> ::windows::core::Result<AddContactResult> {
        let this = &::windows::core::ComInterface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<AddContactResult>();
            (::windows::core::Interface::vtable(this).AddContact)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(contact), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFieldsWithContactFieldType(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::ContactFieldType>> {
        let this = &::windows::core::ComInterface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVector<super::ContactFieldType>>();
            (::windows::core::Interface::vtable(this).DesiredFieldsWithContactFieldType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ContactPickerUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactPickerUI {}
impl ::core::fmt::Debug for ContactPickerUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactPickerUI").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ContactPickerUI {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Provider.ContactPickerUI;{e2cc1366-cf66-43c4-a96a-a5a112db4746})");
}
impl ::core::clone::Clone for ContactPickerUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ContactPickerUI {
    type Vtable = IContactPickerUI_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ContactPickerUI {
    const IID: ::windows::core::GUID = <IContactPickerUI as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ContactPickerUI {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactPickerUI";
}
::windows::imp::interface_hierarchy!(ContactPickerUI, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Contacts_Provider\"`*"]
#[repr(transparent)]
pub struct ContactRemovedEventArgs(::windows::core::IUnknown);
impl ContactRemovedEventArgs {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ContactRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ContactRemovedEventArgs {}
impl ::core::fmt::Debug for ContactRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContactRemovedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for ContactRemovedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs;{6f354338-3302-4d13-ad8d-adcc0ff9e47c})");
}
impl ::core::clone::Clone for ContactRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for ContactRemovedEventArgs {
    type Vtable = IContactRemovedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for ContactRemovedEventArgs {
    const IID: ::windows::core::GUID = <IContactRemovedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for ContactRemovedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs";
}
::windows::imp::interface_hierarchy!(ContactRemovedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"ApplicationModel_Contacts_Provider\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AddContactResult(pub i32);
impl AddContactResult {
    pub const Added: Self = Self(0i32);
    pub const AlreadyAdded: Self = Self(1i32);
    pub const Unavailable: Self = Self(2i32);
}
impl ::core::marker::Copy for AddContactResult {}
impl ::core::clone::Clone for AddContactResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AddContactResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for AddContactResult {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for AddContactResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddContactResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for AddContactResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.Provider.AddContactResult;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
