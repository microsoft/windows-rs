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
unsafe impl ::windows::core::Abi for AddContactResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for AddContactResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddContactResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AddContactResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.Provider.AddContactResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts_Provider\"`*"]
#[repr(transparent)]
pub struct ContactPickerUI(::windows::core::IUnknown);
impl ContactPickerUI {
    #[doc = "*Required features: `\"deprecated\"`*"]
    #[cfg(feature = "deprecated")]
    pub fn AddContact<'a, P0>(&self, id: &::windows::core::HSTRING, contact: P0) -> ::windows::core::Result<AddContactResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Contact>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AddContact)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), contact.into().abi(), result__.as_mut_ptr()).from_abi::<AddContactResult>(result__)
        }
    }
    pub fn RemoveContact(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContact)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id)).ok() }
    }
    pub fn ContainsContact(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContainsContact)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(id), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
    pub fn DesiredFields(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DesiredFields)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn SelectionMode(&self) -> ::windows::core::Result<super::ContactSelectionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectionMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ContactSelectionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContactRemoved<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::TypedEventHandler<ContactPickerUI, ContactRemovedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContactRemoved)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContactRemoved(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContactRemoved)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn AddContact2<'a, P0>(&self, contact: P0) -> ::windows::core::Result<AddContactResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Contact>>,
    {
        let this = &::windows::core::Interface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AddContact)(::windows::core::Interface::as_raw(this), contact.into().abi(), result__.as_mut_ptr()).from_abi::<AddContactResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DesiredFieldsWithContactFieldType(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::ContactFieldType>> {
        let this = &::windows::core::Interface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DesiredFieldsWithContactFieldType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IVector<super::ContactFieldType>>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactPickerUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ContactPickerUI {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Provider.ContactPickerUI;{e2cc1366-cf66-43c4-a96a-a5a112db4746})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContactPickerUI {
    type Vtable = IContactPickerUI_Vtbl;
    const IID: ::windows::core::GUID = <IContactPickerUI as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactPickerUI {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactPickerUI";
}
impl ::core::convert::From<ContactPickerUI> for ::windows::core::IUnknown {
    fn from(value: ContactPickerUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPickerUI> for ::windows::core::IUnknown {
    fn from(value: &ContactPickerUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactPickerUI> for &::windows::core::IUnknown {
    fn from(value: &ContactPickerUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ContactPickerUI> for ::windows::core::IInspectable {
    fn from(value: ContactPickerUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactPickerUI> for ::windows::core::IInspectable {
    fn from(value: &ContactPickerUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactPickerUI> for &::windows::core::IInspectable {
    fn from(value: &ContactPickerUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"ApplicationModel_Contacts_Provider\"`*"]
#[repr(transparent)]
pub struct ContactRemovedEventArgs(::windows::core::IUnknown);
impl ContactRemovedEventArgs {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ContactRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ContactRemovedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs;{6f354338-3302-4d13-ad8d-adcc0ff9e47c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContactRemovedEventArgs {
    type Vtable = IContactRemovedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IContactRemovedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContactRemovedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs";
}
impl ::core::convert::From<ContactRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ContactRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactRemovedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ContactRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactRemovedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &ContactRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ContactRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ContactRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContactRemovedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ContactRemovedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ContactRemovedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &ContactRemovedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContactPickerUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContactPickerUI {
    type Vtable = IContactPickerUI_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2cc1366_cf66_43c4_a96a_a5a112db4746);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerUI_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "deprecated")]
    pub AddContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contact: *mut ::core::ffi::c_void, result__: *mut AddContactResult) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    AddContact: usize,
    pub RemoveContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContainsContact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6e449e28_7b25_4999_9b0b_875400a1e8c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerUI2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6f354338_3302_4d13_ad8d_adcc0ff9e47c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactRemovedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
