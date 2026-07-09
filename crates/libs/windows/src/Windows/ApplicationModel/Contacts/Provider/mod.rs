#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AddContactResult(pub i32);
impl AddContactResult {
    pub const Added: Self = Self(0);
    pub const AlreadyAdded: Self = Self(1);
    pub const Unavailable: Self = Self(2);
}
impl windows_core::TypeKind for AddContactResult {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AddContactResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Contacts.Provider.AddContactResult;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Contacts.Provider.AddContactResult");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactPickerUI(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactPickerUI, windows_core::IUnknown, windows_core::IInspectable);
impl ContactPickerUI {
    pub fn AddContact<P1>(&self, id: &windows_core::HSTRING, contact: P1) -> windows_core::Result<AddContactResult>
    where
        P1: windows_core::Param<super::Contact>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddContact)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(id), contact.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveContact(&self, id: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveContact)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(id)).ok() }
    }
    pub fn ContainsContact(&self, id: &windows_core::HSTRING) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContainsContact)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(id), &mut result__).map(|| result__)
        }
    }
    pub fn DesiredFields(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredFields)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectionMode(&self) -> windows_core::Result<super::ContactSelectionMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectionMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ContactRemoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<ContactRemovedEventArgs>) + Send + 'static,
    {
        let handler = <super::super::super::Foundation::TypedEventHandler<Self, ContactRemovedEventArgs>>::new(move |a0, a1| {
            handler(a0, a1);
            Ok(())
        });
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ContactRemoved)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveContactRemoved))
        }
    }
    pub fn AddContact2<P0>(&self, contact: P0) -> windows_core::Result<AddContactResult>
    where
        P0: windows_core::Param<super::Contact>,
    {
        let this = &windows_core::Interface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddContact)(windows_core::Interface::as_raw(this), contact.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn DesiredFieldsWithContactFieldType(&self) -> windows_core::Result<windows_collections::IVector<super::ContactFieldType>> {
        let this = &windows_core::Interface::cast::<IContactPickerUI2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredFieldsWithContactFieldType)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactPickerUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactPickerUI>();
}
unsafe impl windows_core::Interface for ContactPickerUI {
    type Vtable = <IContactPickerUI as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContactPickerUI as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactPickerUI {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactPickerUI";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactRemovedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContactRemovedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ContactRemovedEventArgs {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for ContactRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContactRemovedEventArgs>();
}
unsafe impl windows_core::Interface for ContactRemovedEventArgs {
    type Vtable = <IContactRemovedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContactRemovedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContactRemovedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs";
}
windows_core::imp::define_interface!(IContactPickerUI, IContactPickerUI_Vtbl, 0xe2cc1366_cf66_43c4_a96a_a5a112db4746);
impl windows_core::RuntimeType for IContactPickerUI {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Contacts.Provider.IContactPickerUI");
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerUI_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut AddContactResult) -> windows_core::HRESULT,
    pub RemoveContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContainsContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub DesiredFields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectionMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ContactSelectionMode) -> windows_core::HRESULT,
    pub ContactRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveContactRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactPickerUI2, IContactPickerUI2_Vtbl, 0x6e449e28_7b25_4999_9b0b_875400a1e8c8);
impl windows_core::RuntimeType for IContactPickerUI2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Contacts.Provider.IContactPickerUI2");
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPickerUI2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut AddContactResult) -> windows_core::HRESULT,
    pub DesiredFieldsWithContactFieldType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactProvider, IContactProvider_Vtbl, 0xc44bb54b_732f_5004_8cd7_65d90cf25f42);
impl windows_core::RuntimeType for IContactProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Contacts.Provider.IContactProvider");
}
windows_core::imp::interface_hierarchy!(IContactProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IContactProvider {
    pub fn GetContactFromRemoteIdAsync(&self, contactremoteid: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::Contact>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContactFromRemoteIdAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(contactremoteid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ContactListId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContactListId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IContactProvider {
    const NAME: &'static str = "Windows.ApplicationModel.Contacts.Provider.IContactProvider";
}
pub trait IContactProvider_Impl: windows_core::IUnknownImpl {
    fn GetContactFromRemoteIdAsync(&self, contactRemoteId: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::Contact>>;
    fn ContactListId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IContactProvider_Vtbl {
    pub const fn new<Identity: IContactProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContactFromRemoteIdAsync<Identity: IContactProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contactremoteid: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactProvider_Impl::GetContactFromRemoteIdAsync(this, core::mem::transmute(&contactremoteid)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContactListId<Identity: IContactProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactProvider_Impl::ContactListId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContactProvider, OFFSET>(),
            GetContactFromRemoteIdAsync: GetContactFromRemoteIdAsync::<Identity, OFFSET>,
            ContactListId: ContactListId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetContactFromRemoteIdAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContactListId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContactRemovedEventArgs, IContactRemovedEventArgs_Vtbl, 0x6f354338_3302_4d13_ad8d_adcc0ff9e47c);
impl windows_core::RuntimeType for IContactRemovedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Contacts.Provider.IContactRemovedEventArgs");
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactRemovedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
