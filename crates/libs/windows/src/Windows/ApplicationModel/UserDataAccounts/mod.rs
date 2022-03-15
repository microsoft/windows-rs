#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub mod Provider;
#[cfg(feature = "ApplicationModel_UserDataAccounts_SystemAccess")]
pub mod SystemAccess;
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccount(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccount {
    type Vtable = IUserDataAccount_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9c4367e_b348_4910_be94_4ad4bba6dea7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccount_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UserDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetUserDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub OtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UserDataAccountOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub SetOtherAppReadAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UserDataAccountOtherAppReadAccess) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Icon: usize,
    pub DeviceAccountTypeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation_Collections"))]
    pub FindAppointmentCalendarsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Appointments", feature = "Foundation_Collections")))]
    FindAppointmentCalendarsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Email", feature = "Foundation_Collections"))]
    pub FindEmailMailboxesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Email", feature = "Foundation_Collections")))]
    FindEmailMailboxesAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub FindContactListsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    FindContactListsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub FindContactAnnotationListsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    FindContactAnnotationListsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccount2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccount2 {
    type Vtable = IUserDataAccount2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x078cd89f_de82_404b_8195_c8a3ac198f60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccount2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub EnterpriseId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsProtectedUnderLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccount3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccount3 {
    type Vtable = IUserDataAccount3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01533845_6c43_4286_9d69_3e1709a1f266);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccount3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub ExplictReadAccessPackageFamilyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExplictReadAccessPackageFamilyNames: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccount4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccount4 {
    type Vtable = IUserDataAccount4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4315210_eae5_4f0a_a8b2_1cca115e008f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccount4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CanShowCreateContactGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCanShowCreateContactGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ProviderProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ProviderProperties: usize,
    #[cfg(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation_Collections"))]
    pub FindUserDataTaskListsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation_Collections")))]
    FindUserDataTaskListsAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub FindContactGroupsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections")))]
    FindContactGroupsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryShowCreateContactGroupAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryShowCreateContactGroupAsync: usize,
    pub SetIsProtectedUnderLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetIcon: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountManagerForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountManagerForUser {
    type Vtable = IUserDataAccountManagerForUser_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56a6e8db_db8f_41ab_a65f_8c5971aac982);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountManagerForUser_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeaccesstype: UserDataAccountStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountManagerStatics {
    type Vtable = IUserDataAccountManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d9b89ea_1928_4a20_86d5_3c737f7dc3b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestStoreAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storeaccesstype: UserDataAccountStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestStoreAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAddAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentkinds: UserDataAccountContentKinds, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAddAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAccountSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAccountSettingsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAccountErrorResolverAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAccountErrorResolverAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountManagerStatics2 {
    type Vtable = IUserDataAccountManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a3ded88_316b_435e_b534_f7d4b4b7dba6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountStore(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountStore {
    type Vtable = IUserDataAccountStore_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2073b0ad_7d0a_4e76_bf45_2368f978a59a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountStore_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAccountsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdisplayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAccountAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountStore2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountStore2 {
    type Vtable = IUserDataAccountStore2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1e0aef7_9560_4631_8af0_061d30161469);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountStore2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAccountWithPackageRelativeAppIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdisplayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagerelativeappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAccountWithPackageRelativeAppIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StoreChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStoreChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStoreChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountStore3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountStore3 {
    type Vtable = IUserDataAccountStore3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8142c094_f3c9_478b_b117_6585bebb6789);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountStore3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdisplayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packagerelativeappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, enterpriseid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountStoreChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountStoreChangedEventArgs {
    type Vtable = IUserDataAccountStoreChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84e3e2e5_8820_4512_b1f6_2e035be1072c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountStoreChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
#[repr(transparent)]
pub struct UserDataAccount(::windows::core::IUnknown);
impl UserDataAccount {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn UserDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserDisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn SetUserDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUserDisplayName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn OtherAppReadAccess(&self) -> ::windows::core::Result<UserDataAccountOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__: UserDataAccountOtherAppReadAccess = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OtherAppReadAccess)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountOtherAppReadAccess>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn SetOtherAppReadAccess(&self, value: UserDataAccountOtherAppReadAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOtherAppReadAccess)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Icon(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Icon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn DeviceAccountTypeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceAccountTypeId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PackageFamilyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"ApplicationModel_Appointments\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation_Collections"))]
    pub fn FindAppointmentCalendarsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Appointments::AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAppointmentCalendarsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Appointments::AppointmentCalendar>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"ApplicationModel_Email\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Email", feature = "Foundation_Collections"))]
    pub fn FindEmailMailboxesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Email::EmailMailbox>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindEmailMailboxesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Email::EmailMailbox>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"ApplicationModel_Contacts\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub fn FindContactListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactList>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindContactListsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactList>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"ApplicationModel_Contacts\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub fn FindContactAnnotationListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactAnnotationList>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindContactAnnotationListsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactAnnotationList>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnterpriseId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn IsProtectedUnderLock(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsProtectedUnderLock)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExplictReadAccessPackageFamilyNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExplictReadAccessPackageFamilyNames)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount3>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn CanShowCreateContactGroup(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanShowCreateContactGroup)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn SetCanShowCreateContactGroup(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCanShowCreateContactGroup)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ProviderProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderProperties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"ApplicationModel_UserDataTasks\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation_Collections"))]
    pub fn FindUserDataTaskListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::UserDataTasks::UserDataTaskList>>> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindUserDataTaskListsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::UserDataTasks::UserDataTaskList>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"ApplicationModel_Contacts\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation_Collections"))]
    pub fn FindContactGroupsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactGroup>>> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindContactGroupsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactGroup>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryShowCreateContactGroupAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryShowCreateContactGroupAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
    pub fn SetIsProtectedUnderLock(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsProtectedUnderLock)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetIcon<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIcon)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UserDataAccount {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccount {}
impl ::core::fmt::Debug for UserDataAccount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccount").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccount {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.UserDataAccount;{b9c4367e-b348-4910-be94-4ad4bba6dea7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataAccount {
    type Vtable = IUserDataAccount_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataAccount as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataAccount {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccount";
}
impl ::core::convert::From<UserDataAccount> for ::windows::core::IUnknown {
    fn from(value: UserDataAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccount> for ::windows::core::IUnknown {
    fn from(value: &UserDataAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccount> for ::windows::core::IInspectable {
    fn from(value: UserDataAccount) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccount> for ::windows::core::IInspectable {
    fn from(value: &UserDataAccount) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataAccount {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccount {}
unsafe impl ::core::marker::Sync for UserDataAccount {}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataAccountContentKinds(pub u32);
impl UserDataAccountContentKinds {
    pub const Email: Self = Self(1u32);
    pub const Contact: Self = Self(2u32);
    pub const Appointment: Self = Self(4u32);
}
impl ::core::marker::Copy for UserDataAccountContentKinds {}
impl ::core::clone::Clone for UserDataAccountContentKinds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataAccountContentKinds {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataAccountContentKinds {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataAccountContentKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountContentKinds").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UserDataAccountContentKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UserDataAccountContentKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UserDataAccountContentKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UserDataAccountContentKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UserDataAccountContentKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountContentKinds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.UserDataAccountContentKinds;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
pub struct UserDataAccountManager {}
impl UserDataAccountManager {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(storeaccesstype: UserDataAccountStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataAccountStore>> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestStoreAsync)(::core::mem::transmute_copy(this), storeaccesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataAccountStore>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAddAccountAsync(contentkinds: UserDataAccountContentKinds) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAddAccountAsync)(::core::mem::transmute_copy(this), contentkinds, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAccountSettingsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAccountSettingsAsync)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAccountErrorResolverAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAccountErrorResolverAsync)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<UserDataAccountManagerForUser> {
        Self::IUserDataAccountManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForUser)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<UserDataAccountManagerForUser>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserDataAccountManagerStatics<R, F: FnOnce(&IUserDataAccountManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataAccountManager, IUserDataAccountManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IUserDataAccountManagerStatics2<R, F: FnOnce(&IUserDataAccountManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataAccountManager, IUserDataAccountManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for UserDataAccountManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountManager";
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
#[repr(transparent)]
pub struct UserDataAccountManagerForUser(::windows::core::IUnknown);
impl UserDataAccountManagerForUser {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(&self, storeaccesstype: UserDataAccountStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataAccountStore>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestStoreAsync)(::core::mem::transmute_copy(this), storeaccesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataAccountStore>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).User)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountManagerForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountManagerForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountManagerForUser {}
impl ::core::fmt::Debug for UserDataAccountManagerForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountManagerForUser").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountManagerForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser;{56a6e8db-db8f-41ab-a65f-8c5971aac982})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataAccountManagerForUser {
    type Vtable = IUserDataAccountManagerForUser_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataAccountManagerForUser as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataAccountManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser";
}
impl ::core::convert::From<UserDataAccountManagerForUser> for ::windows::core::IUnknown {
    fn from(value: UserDataAccountManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountManagerForUser> for ::windows::core::IUnknown {
    fn from(value: &UserDataAccountManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataAccountManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataAccountManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountManagerForUser> for ::windows::core::IInspectable {
    fn from(value: UserDataAccountManagerForUser) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountManagerForUser> for ::windows::core::IInspectable {
    fn from(value: &UserDataAccountManagerForUser) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataAccountManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataAccountManagerForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccountManagerForUser {}
unsafe impl ::core::marker::Sync for UserDataAccountManagerForUser {}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataAccountOtherAppReadAccess(pub i32);
impl UserDataAccountOtherAppReadAccess {
    pub const SystemOnly: Self = Self(0i32);
    pub const Full: Self = Self(1i32);
    pub const None: Self = Self(2i32);
}
impl ::core::marker::Copy for UserDataAccountOtherAppReadAccess {}
impl ::core::clone::Clone for UserDataAccountOtherAppReadAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataAccountOtherAppReadAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataAccountOtherAppReadAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataAccountOtherAppReadAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountOtherAppReadAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountOtherAppReadAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.UserDataAccountOtherAppReadAccess;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
#[repr(transparent)]
pub struct UserDataAccountStore(::windows::core::IUnknown);
impl UserDataAccountStore {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAccountsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataAccount>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindAccountsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataAccount>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, id: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAccountAsync)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataAccount>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdisplayname: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAccountAsync)(::core::mem::transmute_copy(this), userdisplayname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataAccount>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAccountWithPackageRelativeAppIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdisplayname: Param0, packagerelativeappid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>> {
        let this = &::windows::core::Interface::cast::<IUserDataAccountStore2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAccountWithPackageRelativeAppIdAsync)(::core::mem::transmute_copy(this), userdisplayname.into_param().abi(), packagerelativeappid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataAccount>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StoreChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<UserDataAccountStore, UserDataAccountStoreChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IUserDataAccountStore2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StoreChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStoreChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IUserDataAccountStore2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStoreChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdisplayname: Param0, packagerelativeappid: Param1, enterpriseid: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>> {
        let this = &::windows::core::Interface::cast::<IUserDataAccountStore3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync)(::core::mem::transmute_copy(this), userdisplayname.into_param().abi(), packagerelativeappid.into_param().abi(), enterpriseid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataAccount>>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountStore {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountStore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountStore {}
impl ::core::fmt::Debug for UserDataAccountStore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountStore").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore;{2073b0ad-7d0a-4e76-bf45-2368f978a59a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataAccountStore {
    type Vtable = IUserDataAccountStore_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataAccountStore as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataAccountStore {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore";
}
impl ::core::convert::From<UserDataAccountStore> for ::windows::core::IUnknown {
    fn from(value: UserDataAccountStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountStore> for ::windows::core::IUnknown {
    fn from(value: &UserDataAccountStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataAccountStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataAccountStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountStore> for ::windows::core::IInspectable {
    fn from(value: UserDataAccountStore) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountStore> for ::windows::core::IInspectable {
    fn from(value: &UserDataAccountStore) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataAccountStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataAccountStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccountStore {}
unsafe impl ::core::marker::Sync for UserDataAccountStore {}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UserDataAccountStoreAccessType(pub i32);
impl UserDataAccountStoreAccessType {
    pub const AllAccountsReadOnly: Self = Self(0i32);
    pub const AppAccountsReadWrite: Self = Self(1i32);
}
impl ::core::marker::Copy for UserDataAccountStoreAccessType {}
impl ::core::clone::Clone for UserDataAccountStoreAccessType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UserDataAccountStoreAccessType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UserDataAccountStoreAccessType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UserDataAccountStoreAccessType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountStoreAccessType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountStoreAccessType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreAccessType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`*"]
#[repr(transparent)]
pub struct UserDataAccountStoreChangedEventArgs(::windows::core::IUnknown);
impl UserDataAccountStoreChangedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataAccountStoreChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataAccountStoreChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataAccountStoreChangedEventArgs {}
impl ::core::fmt::Debug for UserDataAccountStoreChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataAccountStoreChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataAccountStoreChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs;{84e3e2e5-8820-4512-b1f6-2e035be1072c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UserDataAccountStoreChangedEventArgs {
    type Vtable = IUserDataAccountStoreChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IUserDataAccountStoreChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataAccountStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs";
}
impl ::core::convert::From<UserDataAccountStoreChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: UserDataAccountStoreChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountStoreChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UserDataAccountStoreChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataAccountStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataAccountStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UserDataAccountStoreChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: UserDataAccountStoreChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UserDataAccountStoreChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UserDataAccountStoreChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataAccountStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataAccountStoreChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UserDataAccountStoreChangedEventArgs {}
unsafe impl ::core::marker::Sync for UserDataAccountStoreChangedEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
