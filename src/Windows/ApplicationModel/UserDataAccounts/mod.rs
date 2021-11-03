#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_UserDataAccounts_Provider")]
pub mod Provider;
#[cfg(feature = "ApplicationModel_UserDataAccounts_SystemAccess")]
pub mod SystemAccess;
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccount(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccount {
    type Vtable = IUserDataAccount_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3116643966, 45896, 18704, [190, 148, 74, 212, 187, 166, 222, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccount_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataAccountOtherAppReadAccess) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UserDataAccountOtherAppReadAccess) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Appointments", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel_Email", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Email", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccount2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccount2 {
    type Vtable = IUserDataAccount2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(126671007, 56962, 16459, [129, 149, 200, 163, 172, 25, 143, 96]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccount2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccount3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccount3 {
    type Vtable = IUserDataAccount3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(22231109, 27715, 17030, [157, 105, 62, 23, 9, 161, 242, 102]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccount3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccount4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccount4 {
    type Vtable = IUserDataAccount4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3291566608, 60133, 20234, [168, 178, 28, 202, 17, 94, 0, 143]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccount4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Contacts", feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountManagerForUser(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountManagerForUser {
    type Vtable = IUserDataAccountManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1453779163, 56207, 16811, [166, 95, 140, 89, 113, 170, 201, 130]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountManagerForUser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storeaccesstype: UserDataAccountStoreAccessType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountManagerStatics {
    type Vtable = IUserDataAccountManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(228297194, 6440, 18976, [134, 213, 60, 115, 127, 125, 195, 176]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storeaccesstype: UserDataAccountStoreAccessType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentkinds: UserDataAccountContentKinds, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountManagerStatics2 {
    type Vtable = IUserDataAccountManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1782443400, 12651, 17246, [181, 52, 247, 212, 180, 183, 219, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountStore(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountStore {
    type Vtable = IUserDataAccountStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(544452781, 32010, 20086, [191, 69, 35, 104, 249, 120, 165, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdisplayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountStore2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountStore2 {
    type Vtable = IUserDataAccountStore2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2984292087, 38240, 17969, [138, 240, 6, 29, 48, 22, 20, 105]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountStore2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdisplayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagerelativeappid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountStore3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountStore3 {
    type Vtable = IUserDataAccountStore3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2168635540, 62409, 18315, [177, 23, 101, 133, 190, 187, 103, 137]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountStore3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdisplayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, packagerelativeappid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, enterpriseid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountStoreChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountStoreChangedEventArgs {
    type Vtable = IUserDataAccountStoreChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2229527269, 34848, 17682, [177, 246, 46, 3, 91, 225, 7, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountStoreChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserDataAccount(::windows::runtime::IInspectable);
impl UserDataAccount {
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn UserDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn SetUserDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn OtherAppReadAccess(&self) -> ::windows::runtime::Result<UserDataAccountOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__: UserDataAccountOtherAppReadAccess = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataAccountOtherAppReadAccess>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn SetOtherAppReadAccess(&self, value: UserDataAccountOtherAppReadAccess) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Storage_Streams`*"]
    pub fn Icon(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn DeviceAccountTypeId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn PackageFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn SaveAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn DeleteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Appointments", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `ApplicationModel_Appointments`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAppointmentCalendarsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Appointments::AppointmentCalendar>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Appointments::AppointmentCalendar>>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Email", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `ApplicationModel_Email`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindEmailMailboxesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Email::EmailMailbox>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Email::EmailMailbox>>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindContactListsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactList>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactList>>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindContactAnnotationListsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactAnnotationList>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactAnnotationList>>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn EnterpriseId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn IsProtectedUnderLock(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation_Collections`*"]
    pub fn ExplictReadAccessPackageFamilyNames(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn CanShowCreateContactGroup(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn SetCanShowCreateContactGroup(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation_Collections`*"]
    pub fn ProviderProperties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_UserDataTasks", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `ApplicationModel_UserDataTasks`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindUserDataTaskListsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::UserDataTasks::UserDataTaskList>>> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::UserDataTasks::UserDataTaskList>>>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Contacts", feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `ApplicationModel_Contacts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindContactGroupsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactGroup>>> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::Contacts::ContactGroup>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn TryShowCreateContactGroupAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
    pub fn SetIsProtectedUnderLock(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Storage_Streams`*"]
    pub fn SetIcon<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamReference>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccount4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccount {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.UserDataAccount;{b9c4367e-b348-4910-be94-4ad4bba6dea7})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccount {
    type Vtable = IUserDataAccount_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3116643966, 45896, 18704, [190, 148, 74, 212, 187, 166, 222, 167]);
}
impl ::windows::runtime::RuntimeName for UserDataAccount {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccount";
}
impl ::std::convert::From<UserDataAccount> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccount) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserDataAccount> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccount) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserDataAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserDataAccount> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccount) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserDataAccount> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataAccount) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataAccount {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UserDataAccount {}
unsafe impl ::std::marker::Sync for UserDataAccount {}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataAccountContentKinds(pub u32);
impl UserDataAccountContentKinds {
    pub const Email: UserDataAccountContentKinds = UserDataAccountContentKinds(1u32);
    pub const Contact: UserDataAccountContentKinds = UserDataAccountContentKinds(2u32);
    pub const Appointment: UserDataAccountContentKinds = UserDataAccountContentKinds(4u32);
}
impl ::std::convert::From<u32> for UserDataAccountContentKinds {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataAccountContentKinds {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountContentKinds {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.UserDataAccountContentKinds;u4)");
}
impl ::windows::runtime::DefaultType for UserDataAccountContentKinds {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for UserDataAccountContentKinds {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for UserDataAccountContentKinds {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for UserDataAccountContentKinds {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for UserDataAccountContentKinds {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for UserDataAccountContentKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
pub struct UserDataAccountManager {}
impl UserDataAccountManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn RequestStoreAsync(storeaccesstype: UserDataAccountStoreAccessType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataAccountStore>> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), storeaccesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataAccountStore>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn ShowAddAccountAsync(contentkinds: UserDataAccountContentKinds) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), contentkinds, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn ShowAccountSettingsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(id: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn ShowAccountErrorResolverAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(id: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IUserDataAccountManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<UserDataAccountManagerForUser> {
        Self::IUserDataAccountManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<UserDataAccountManagerForUser>(result__)
        })
    }
    pub fn IUserDataAccountManagerStatics<R, F: FnOnce(&IUserDataAccountManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserDataAccountManager, IUserDataAccountManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUserDataAccountManagerStatics2<R, F: FnOnce(&IUserDataAccountManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserDataAccountManager, IUserDataAccountManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for UserDataAccountManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountManager";
}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserDataAccountManagerForUser(::windows::runtime::IInspectable);
impl UserDataAccountManagerForUser {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn RequestStoreAsync(&self, storeaccesstype: UserDataAccountStoreAccessType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataAccountStore>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), storeaccesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataAccountStore>>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountManagerForUser {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser;{56a6e8db-db8f-41ab-a65f-8c5971aac982})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccountManagerForUser {
    type Vtable = IUserDataAccountManagerForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1453779163, 56207, 16811, [166, 95, 140, 89, 113, 170, 201, 130]);
}
impl ::windows::runtime::RuntimeName for UserDataAccountManagerForUser {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountManagerForUser";
}
impl ::std::convert::From<UserDataAccountManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccountManagerForUser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserDataAccountManagerForUser> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccountManagerForUser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccountManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserDataAccountManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserDataAccountManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccountManagerForUser) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserDataAccountManagerForUser> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataAccountManagerForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataAccountManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataAccountManagerForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UserDataAccountManagerForUser {}
unsafe impl ::std::marker::Sync for UserDataAccountManagerForUser {}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataAccountOtherAppReadAccess(pub i32);
impl UserDataAccountOtherAppReadAccess {
    pub const SystemOnly: UserDataAccountOtherAppReadAccess = UserDataAccountOtherAppReadAccess(0i32);
    pub const Full: UserDataAccountOtherAppReadAccess = UserDataAccountOtherAppReadAccess(1i32);
    pub const None: UserDataAccountOtherAppReadAccess = UserDataAccountOtherAppReadAccess(2i32);
}
impl ::std::convert::From<i32> for UserDataAccountOtherAppReadAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataAccountOtherAppReadAccess {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountOtherAppReadAccess {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.UserDataAccountOtherAppReadAccess;i4)");
}
impl ::windows::runtime::DefaultType for UserDataAccountOtherAppReadAccess {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserDataAccountStore(::windows::runtime::IInspectable);
impl UserDataAccountStore {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindAccountsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataAccount>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataAccount>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn GetAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, id: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataAccount>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn CreateAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, userdisplayname: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), userdisplayname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataAccount>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn CreateAccountWithPackageRelativeAppIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, userdisplayname: Param0, packagerelativeappid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccountStore2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), userdisplayname.into_param().abi(), packagerelativeappid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataAccount>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn StoreChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<UserDataAccountStore, UserDataAccountStoreChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccountStore2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn RemoveStoreChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccountStore2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn CreateAccountWithPackageRelativeAppIdAndEnterpriseIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, userdisplayname: Param0, packagerelativeappid: Param1, enterpriseid: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataAccount>> {
        let this = &::windows::runtime::Interface::cast::<IUserDataAccountStore3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), userdisplayname.into_param().abi(), packagerelativeappid.into_param().abi(), enterpriseid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataAccount>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountStore {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore;{2073b0ad-7d0a-4e76-bf45-2368f978a59a})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccountStore {
    type Vtable = IUserDataAccountStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(544452781, 32010, 20086, [191, 69, 35, 104, 249, 120, 165, 154]);
}
impl ::windows::runtime::RuntimeName for UserDataAccountStore {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountStore";
}
impl ::std::convert::From<UserDataAccountStore> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccountStore) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserDataAccountStore> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccountStore) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccountStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserDataAccountStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserDataAccountStore> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccountStore) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserDataAccountStore> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataAccountStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataAccountStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataAccountStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UserDataAccountStore {}
unsafe impl ::std::marker::Sync for UserDataAccountStore {}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataAccountStoreAccessType(pub i32);
impl UserDataAccountStoreAccessType {
    pub const AllAccountsReadOnly: UserDataAccountStoreAccessType = UserDataAccountStoreAccessType(0i32);
    pub const AppAccountsReadWrite: UserDataAccountStoreAccessType = UserDataAccountStoreAccessType(1i32);
}
impl ::std::convert::From<i32> for UserDataAccountStoreAccessType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataAccountStoreAccessType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountStoreAccessType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreAccessType;i4)");
}
impl ::windows::runtime::DefaultType for UserDataAccountStoreAccessType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserDataAccountStoreChangedEventArgs(::windows::runtime::IInspectable);
impl UserDataAccountStoreChangedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataAccountStoreChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs;{84e3e2e5-8820-4512-b1f6-2e035be1072c})");
}
unsafe impl ::windows::runtime::Interface for UserDataAccountStoreChangedEventArgs {
    type Vtable = IUserDataAccountStoreChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2229527269, 34848, 17682, [177, 246, 46, 3, 91, 225, 7, 44]);
}
impl ::windows::runtime::RuntimeName for UserDataAccountStoreChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.UserDataAccountStoreChangedEventArgs";
}
impl ::std::convert::From<UserDataAccountStoreChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: UserDataAccountStoreChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserDataAccountStoreChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &UserDataAccountStoreChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserDataAccountStoreChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserDataAccountStoreChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserDataAccountStoreChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: UserDataAccountStoreChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserDataAccountStoreChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &UserDataAccountStoreChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserDataAccountStoreChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserDataAccountStoreChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UserDataAccountStoreChangedEventArgs {}
unsafe impl ::std::marker::Sync for UserDataAccountStoreChangedEventArgs {}
