#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IContactPickerUIImpl: Sized {
    fn AddContact(&mut self, id: &::windows::core::HSTRING, contact: &::core::option::Option<super::Contact>) -> ::windows::core::Result<AddContactResult>;
    fn RemoveContact(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContainsContact(&mut self, id: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn DesiredFields(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SelectionMode(&mut self) -> ::windows::core::Result<super::ContactSelectionMode>;
    fn ContactRemoved(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<ContactPickerUI, ContactRemovedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveContactRemoved(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactPickerUI {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.IContactPickerUI";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IContactPickerUIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPickerUIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactPickerUIVtbl {
        unsafe extern "system" fn AddContact<Impl: IContactPickerUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, contact: ::windows::core::RawPtr, result__: *mut AddContactResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddContact(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&contact as *const <super::Contact as ::windows::core::Abi>::Abi as *const <super::Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContact<Impl: IContactPickerUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContact(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContainsContact<Impl: IContactPickerUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainsContact(&*(&id as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredFields<Impl: IContactPickerUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredFields() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionMode<Impl: IContactPickerUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ContactSelectionMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContactRemoved<Impl: IContactPickerUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactRemoved(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<ContactPickerUI, ContactRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<ContactPickerUI, ContactRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContactRemoved<Impl: IContactPickerUIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContactRemoved(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactPickerUI, BASE_OFFSET>(),
            AddContact: AddContact::<Impl, IMPL_OFFSET>,
            RemoveContact: RemoveContact::<Impl, IMPL_OFFSET>,
            ContainsContact: ContainsContact::<Impl, IMPL_OFFSET>,
            DesiredFields: DesiredFields::<Impl, IMPL_OFFSET>,
            SelectionMode: SelectionMode::<Impl, IMPL_OFFSET>,
            ContactRemoved: ContactRemoved::<Impl, IMPL_OFFSET>,
            RemoveContactRemoved: RemoveContactRemoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactPickerUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IContactPickerUI2Impl: Sized {
    fn AddContact(&mut self, contact: &::core::option::Option<super::Contact>) -> ::windows::core::Result<AddContactResult>;
    fn DesiredFieldsWithContactFieldType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::ContactFieldType>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContactPickerUI2 {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.IContactPickerUI2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IContactPickerUI2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactPickerUI2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactPickerUI2Vtbl {
        unsafe extern "system" fn AddContact<Impl: IContactPickerUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contact: ::windows::core::RawPtr, result__: *mut AddContactResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddContact(&*(&contact as *const <super::Contact as ::windows::core::Abi>::Abi as *const <super::Contact as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredFieldsWithContactFieldType<Impl: IContactPickerUI2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredFieldsWithContactFieldType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContactPickerUI2, BASE_OFFSET>(),
            AddContact: AddContact::<Impl, IMPL_OFFSET>,
            DesiredFieldsWithContactFieldType: DesiredFieldsWithContactFieldType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactPickerUI2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContactRemovedEventArgsImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContactRemovedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.IContactRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IContactRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContactRemovedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContactRemovedEventArgsVtbl {
        unsafe extern "system" fn Id<Impl: IContactRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IContactRemovedEventArgs, BASE_OFFSET>(), Id: Id::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
