pub trait IContactField_Impl: Sized {
    fn Type(&self) -> ::windows::core::Result<ContactFieldType>;
    fn Category(&self) -> ::windows::core::Result<ContactFieldCategory>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IContactField {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactField";
}
impl IContactField_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: isize>() -> IContactField_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactFieldType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Category<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ContactFieldCategory) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Category() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactField_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IContactField, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Category: Category::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactField as ::windows::core::Interface>::IID
    }
}
pub trait IContactFieldFactory_Impl: Sized {
    fn CreateField_Default(&self, value: &::windows::core::HSTRING, r#type: ContactFieldType) -> ::windows::core::Result<ContactField>;
    fn CreateField_Category(&self, value: &::windows::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::core::Result<ContactField>;
    fn CreateField_Custom(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, r#type: ContactFieldType, category: ContactFieldCategory) -> ::windows::core::Result<ContactField>;
}
impl ::windows::core::RuntimeName for IContactFieldFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactFieldFactory";
}
impl IContactFieldFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactFieldFactory_Impl, const OFFSET: isize>() -> IContactFieldFactory_Vtbl {
        unsafe extern "system" fn CreateField_Default<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactFieldFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, r#type: ContactFieldType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateField_Default(::core::mem::transmute(&value), r#type) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateField_Category<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactFieldFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateField_Category(::core::mem::transmute(&value), r#type, category) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateField_Custom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactFieldFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, r#type: ContactFieldType, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateField_Custom(::core::mem::transmute(&name), ::core::mem::transmute(&value), r#type, category) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IContactFieldFactory, OFFSET>(),
            CreateField_Default: CreateField_Default::<Identity, Impl, OFFSET>,
            CreateField_Category: CreateField_Category::<Identity, Impl, OFFSET>,
            CreateField_Custom: CreateField_Custom::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactFieldFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IContactInstantMessageFieldFactory_Impl: Sized {
    fn CreateInstantMessage_Default(&self, username: &::windows::core::HSTRING) -> ::windows::core::Result<ContactInstantMessageField>;
    fn CreateInstantMessage_Category(&self, username: &::windows::core::HSTRING, category: ContactFieldCategory) -> ::windows::core::Result<ContactInstantMessageField>;
    fn CreateInstantMessage_All(&self, username: &::windows::core::HSTRING, category: ContactFieldCategory, service: &::windows::core::HSTRING, displaytext: &::windows::core::HSTRING, verb: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<ContactInstantMessageField>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IContactInstantMessageFieldFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactInstantMessageFieldFactory";
}
#[cfg(feature = "Foundation")]
impl IContactInstantMessageFieldFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactInstantMessageFieldFactory_Impl, const OFFSET: isize>() -> IContactInstantMessageFieldFactory_Vtbl {
        unsafe extern "system" fn CreateInstantMessage_Default<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactInstantMessageFieldFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstantMessage_Default(::core::mem::transmute(&username)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstantMessage_Category<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactInstantMessageFieldFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstantMessage_Category(::core::mem::transmute(&username), category) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstantMessage_All<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactInstantMessageFieldFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, username: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: ContactFieldCategory, service: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displaytext: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, verb: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstantMessage_All(::core::mem::transmute(&username), category, ::core::mem::transmute(&service), ::core::mem::transmute(&displaytext), ::core::mem::transmute(&verb)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IContactInstantMessageFieldFactory, OFFSET>(),
            CreateInstantMessage_Default: CreateInstantMessage_Default::<Identity, Impl, OFFSET>,
            CreateInstantMessage_Category: CreateInstantMessage_Category::<Identity, Impl, OFFSET>,
            CreateInstantMessage_All: CreateInstantMessage_All::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactInstantMessageFieldFactory as ::windows::core::Interface>::IID
    }
}
pub trait IContactLocationFieldFactory_Impl: Sized {
    fn CreateLocation_Default(&self, unstructuredaddress: &::windows::core::HSTRING) -> ::windows::core::Result<ContactLocationField>;
    fn CreateLocation_Category(&self, unstructuredaddress: &::windows::core::HSTRING, category: ContactFieldCategory) -> ::windows::core::Result<ContactLocationField>;
    fn CreateLocation_All(&self, unstructuredaddress: &::windows::core::HSTRING, category: ContactFieldCategory, street: &::windows::core::HSTRING, city: &::windows::core::HSTRING, region: &::windows::core::HSTRING, country: &::windows::core::HSTRING, postalcode: &::windows::core::HSTRING) -> ::windows::core::Result<ContactLocationField>;
}
impl ::windows::core::RuntimeName for IContactLocationFieldFactory {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.IContactLocationFieldFactory";
}
impl IContactLocationFieldFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactLocationFieldFactory_Impl, const OFFSET: isize>() -> IContactLocationFieldFactory_Vtbl {
        unsafe extern "system" fn CreateLocation_Default<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactLocationFieldFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unstructuredaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateLocation_Default(::core::mem::transmute(&unstructuredaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLocation_Category<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactLocationFieldFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unstructuredaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: ContactFieldCategory, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateLocation_Category(::core::mem::transmute(&unstructuredaddress), category) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateLocation_All<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContactLocationFieldFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unstructuredaddress: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, category: ContactFieldCategory, street: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, city: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, region: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, country: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, postalcode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateLocation_All(::core::mem::transmute(&unstructuredaddress), category, ::core::mem::transmute(&street), ::core::mem::transmute(&city), ::core::mem::transmute(&region), ::core::mem::transmute(&country), ::core::mem::transmute(&postalcode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IContactLocationFieldFactory, OFFSET>(),
            CreateLocation_Default: CreateLocation_Default::<Identity, Impl, OFFSET>,
            CreateLocation_Category: CreateLocation_Category::<Identity, Impl, OFFSET>,
            CreateLocation_All: CreateLocation_All::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContactLocationFieldFactory as ::windows::core::Interface>::IID
    }
}
