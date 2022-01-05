#[cfg(feature = "implement_exclusive")]
pub trait IContactPickerUIImpl: Sized {
    fn AddContact(&self, id: &::windows::core::HSTRING, contact: &::core::option::Option<super::Contact>) -> ::windows::core::Result<AddContactResult>;
    fn RemoveContact(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContainsContact(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn DesiredFields(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SelectionMode(&self) -> ::windows::core::Result<super::ContactSelectionMode>;
    fn ContactRemoved(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactPickerUI, ContactRemovedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveContactRemoved(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPickerUI2Impl: Sized {
    fn AddContact(&self, contact: &::core::option::Option<super::Contact>) -> ::windows::core::Result<AddContactResult>;
    fn DesiredFieldsWithContactFieldType(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::ContactFieldType>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactRemovedEventArgsImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
