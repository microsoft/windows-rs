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
impl ::windows::core::RuntimeName for IContactPickerUI {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.IContactPickerUI";
}
#[cfg(feature = "implement_exclusive")]
impl IContactPickerUIVtbl {
    pub const fn new<Impl: IContactPickerUIImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactPickerUIVtbl {
        unsafe extern "system" fn AddContact<Impl: IContactPickerUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contact: ::windows::core::RawPtr, result__: *mut AddContactResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddContact(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&contact as *const <super::Contact as ::windows::core::Abi>::Abi as *const <super::Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContact<Impl: IContactPickerUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveContact(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContainsContact<Impl: IContactPickerUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContainsContact(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredFields<Impl: IContactPickerUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredFields() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionMode<Impl: IContactPickerUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::ContactSelectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContactRemoved<Impl: IContactPickerUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContactRemoved(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<ContactPickerUI, ContactRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<ContactPickerUI, ContactRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContactRemoved<Impl: IContactPickerUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveContactRemoved(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactPickerUI>, base.5, AddContact::<Impl, OFFSET>, RemoveContact::<Impl, OFFSET>, ContainsContact::<Impl, OFFSET>, DesiredFields::<Impl, OFFSET>, SelectionMode::<Impl, OFFSET>, ContactRemoved::<Impl, OFFSET>, RemoveContactRemoved::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactPickerUI2Impl: Sized {
    fn AddContact(&self, contact: &::core::option::Option<super::Contact>) -> ::windows::core::Result<AddContactResult>;
    fn DesiredFieldsWithContactFieldType(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::ContactFieldType>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactPickerUI2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.IContactPickerUI2";
}
#[cfg(feature = "implement_exclusive")]
impl IContactPickerUI2Vtbl {
    pub const fn new<Impl: IContactPickerUI2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactPickerUI2Vtbl {
        unsafe extern "system" fn AddContact<Impl: IContactPickerUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut AddContactResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddContact(&*(&contact as *const <super::Contact as ::windows::core::Abi>::Abi as *const <super::Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredFieldsWithContactFieldType<Impl: IContactPickerUI2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredFieldsWithContactFieldType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactPickerUI2>, base.5, AddContact::<Impl, OFFSET>, DesiredFieldsWithContactFieldType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactRemovedEventArgsImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactRemovedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.IContactRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IContactRemovedEventArgsVtbl {
    pub const fn new<Impl: IContactRemovedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContactRemovedEventArgsVtbl {
        unsafe extern "system" fn Id<Impl: IContactRemovedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContactRemovedEventArgs>, base.5, Id::<Impl, OFFSET>)
    }
}
