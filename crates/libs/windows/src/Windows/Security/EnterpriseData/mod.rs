#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IBufferProtectUnprotectResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBufferProtectUnprotectResult {
    type Vtable = IBufferProtectUnprotectResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IBufferProtectUnprotectResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x47995edc_6cec_4e3a_b251_9e7485d79e7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferProtectUnprotectResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    pub ProtectionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDataProtectionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataProtectionInfo {
    type Vtable = IDataProtectionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataProtectionInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8420b0c1_5e31_4405_9540_3f943af0cb26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DataProtectionStatus) -> ::windows_core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IDataProtectionManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataProtectionManagerStatics {
    type Vtable = IDataProtectionManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IDataProtectionManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6149b74_9144_4ee4_8a8a_30b5f361430e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataProtectionManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ProtectStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unprotectedstream: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, protectedstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ProtectStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub UnprotectStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedstream: *mut ::core::ffi::c_void, unprotectedstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    UnprotectStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetProtectionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protecteddata: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetProtectionInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetStreamProtectionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetStreamProtectionInfoAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileProtectionInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileProtectionInfo {
    type Vtable = IFileProtectionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileProtectionInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4ee96486_147e_4dd0_8faf_5253ed91ad0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FileProtectionStatus) -> ::windows_core::HRESULT,
    pub IsRoamable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileProtectionInfo2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileProtectionInfo2 {
    type Vtable = IFileProtectionInfo2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileProtectionInfo2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82123a4c_557a_498d_8e94_944cd5836432);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionInfo2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsProtectWhileOpenSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileProtectionManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileProtectionManagerStatics {
    type Vtable = IFileProtectionManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileProtectionManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5846fc9b_e613_426b_bb38_88cba1dc9adb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CopyProtectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CopyProtectionAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetProtectionInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetProtectionInfoAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub SaveFileAsContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    SaveFileAsContainerAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFileFromContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerfile: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFileFromContainerAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFileFromContainerWithTargetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerfile: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFileFromContainerWithTargetAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub CreateProtectedAndOpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentfolder: *mut ::core::ffi::c_void, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, collisionoption: super::super::Storage::CreationCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    CreateProtectedAndOpenAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileProtectionManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileProtectionManagerStatics2 {
    type Vtable = IFileProtectionManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileProtectionManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x83d2a745_0483_41ab_b2d5_bc7f23d74ebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub IsContainerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    IsContainerAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub LoadFileFromContainerWithTargetAndNameCollisionOptionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerfile: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, collisionoption: super::super::Storage::NameCollisionOption, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    LoadFileFromContainerWithTargetAndNameCollisionOptionAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub SaveFileAsContainerWithSharingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protectedfile: *mut ::core::ffi::c_void, sharedwithidentities: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    SaveFileAsContainerWithSharingAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileProtectionManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileProtectionManagerStatics3 {
    type Vtable = IFileProtectionManagerStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileProtectionManagerStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6918849a_624f_46d6_b241_e9cd5fdf3e3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileProtectionManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub UnprotectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    UnprotectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub UnprotectWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    UnprotectWithOptionsAsync: usize,
}
#[doc(hidden)]
#[cfg(feature = "deprecated")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileRevocationManagerStatics(::windows_core::IUnknown);
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::Interface for IFileRevocationManagerStatics {
    type Vtable = IFileRevocationManagerStatics_Vtbl;
}
#[cfg(feature = "deprecated")]
unsafe impl ::windows_core::ComInterface for IFileRevocationManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x256bbc3d_1c5d_4260_8c75_9144cfb78ba9);
}
#[cfg(feature = "deprecated")]
#[repr(C)]
#[doc(hidden)]
pub struct IFileRevocationManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub ProtectAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storageitem: *mut ::core::ffi::c_void, enterpriseidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    ProtectAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub CopyProtectionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestorageitem: *mut ::core::ffi::c_void, targetstorageitem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    CopyProtectionAsync: usize,
    #[cfg(feature = "deprecated")]
    pub Revoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enterpriseidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "deprecated"))]
    Revoke: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub GetStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storageitem: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage", feature = "deprecated")))]
    GetStatusAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileUnprotectOptions(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileUnprotectOptions {
    type Vtable = IFileUnprotectOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileUnprotectOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7d1312f1_3b0d_4dd8_a1f8_1ec53822e2f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUnprotectOptions_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetAudit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Audit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IFileUnprotectOptionsFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFileUnprotectOptionsFactory {
    type Vtable = IFileUnprotectOptionsFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IFileUnprotectOptionsFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51aeb39c_da8c_4c3f_9bfb_cb73a7cce0dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUnprotectOptionsFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, audit: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectedAccessResumedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectedAccessResumedEventArgs {
    type Vtable = IProtectedAccessResumedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectedAccessResumedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac4dca59_5d80_4e95_8c5f_8539450eebe0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedAccessResumedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Identities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Identities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectedAccessSuspendingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectedAccessSuspendingEventArgs {
    type Vtable = IProtectedAccessSuspendingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectedAccessSuspendingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75a193e0_a344_429f_b975_04fc1f88c185);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedAccessSuspendingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Identities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Identities: usize,
    #[cfg(feature = "Foundation")]
    pub Deadline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Deadline: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectedContainerExportResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectedContainerExportResult {
    type Vtable = IProtectedContainerExportResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectedContainerExportResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3948ef95_f7fb_4b42_afb0_df70b41543c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContainerExportResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProtectedImportExportStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectedContainerImportResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectedContainerImportResult {
    type Vtable = IProtectedContainerImportResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectedContainerImportResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcdb780d1_e7bb_4d1a_9339_34dc41149f9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContainerImportResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProtectedImportExportStatus) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectedContentRevokedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectedContentRevokedEventArgs {
    type Vtable = IProtectedContentRevokedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectedContentRevokedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x63686821_58b9_47ee_93d9_f0f741cf43f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedContentRevokedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Identities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Identities: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectedFileCreateResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectedFileCreateResult {
    type Vtable = IProtectedFileCreateResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectedFileCreateResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x28e3ed6a_e9e7_4a03_9f53_bdb16172699b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectedFileCreateResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage")]
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    File: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    pub ProtectionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectionPolicyAuditInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyAuditInfo {
    type Vtable = IProtectionPolicyAuditInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectionPolicyAuditInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x425ab7e4_feb7_44fc_b3bb_c3c4d7ecbebb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ProtectionPolicyAuditAction) -> ::windows_core::HRESULT,
    pub Action: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ProtectionPolicyAuditAction) -> ::windows_core::HRESULT,
    pub SetDataDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DataDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSourceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SourceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTargetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TargetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectionPolicyAuditInfoFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyAuditInfoFactory {
    type Vtable = IProtectionPolicyAuditInfoFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectionPolicyAuditInfoFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ed4180b_92e8_42d5_83d4_25440b423549);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyAuditInfoFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: ProtectionPolicyAuditAction, datadescription: ::std::mem::MaybeUninit<::windows_core::HSTRING>, sourcedescription: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetdescription: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithActionAndDataDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, action: ProtectionPolicyAuditAction, datadescription: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectionPolicyManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyManager {
    type Vtable = IProtectionPolicyManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectionPolicyManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd5703e18_a08d_47e6_a240_9934d7165eb5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectionPolicyManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyManager2 {
    type Vtable = IProtectionPolicyManager2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectionPolicyManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xabf7527a_8435_417f_99b6_51beaf365888);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetShowEnterpriseIndicator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub ShowEnterpriseIndicator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectionPolicyManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyManagerStatics {
    type Vtable = IProtectionPolicyManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectionPolicyManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0bffc66_8c3d_4d56_8804_c68f0ad32ec5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsIdentityManaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub TryApplyProcessUIPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ClearProcessUIPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateCurrentThreadNetworkContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub GetPrimaryManagedIdentityForNetworkEndpointAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpointhost: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))]
    GetPrimaryManagedIdentityForNetworkEndpointAsync: usize,
    pub RevokeContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ProtectedAccessSuspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtectedAccessSuspending: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProtectedAccessSuspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProtectedAccessSuspending: usize,
    #[cfg(feature = "Foundation")]
    pub ProtectedAccessResumed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtectedAccessResumed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProtectedAccessResumed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProtectedAccessResumed: usize,
    #[cfg(feature = "Foundation")]
    pub ProtectedContentRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ProtectedContentRevoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProtectedContentRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProtectedContentRevoked: usize,
    pub CheckAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ProtectionPolicyEvaluationResult) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectionPolicyManagerStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyManagerStatics2 {
    type Vtable = IProtectionPolicyManagerStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectionPolicyManagerStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb68f9a8c_39e0_4649_b2e4_070ab8a579b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub HasContentBeenRevokedSince: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, since: super::super::Foundation::DateTime, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HasContentBeenRevokedSince: usize,
    pub CheckAccessForApp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ProtectionPolicyEvaluationResult) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForAppAsync: usize,
    pub GetEnforcementLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut EnforcementLevel) -> ::windows_core::HRESULT,
    pub IsUserDecryptionAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsProtectionUnderLockRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PolicyChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePolicyChanged: usize,
    pub IsProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectionPolicyManagerStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyManagerStatics3 {
    type Vtable = IProtectionPolicyManagerStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectionPolicyManagerStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48ff9e8c_6a6f_4d9f_bced_18ab537aa015);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RequestAccessWithAuditingInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessWithAuditingInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessWithMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfo: *mut ::core::ffi::c_void, messagefromapp: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessWithMessageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForAppWithAuditingInfoAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForAppWithAuditingInfoAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForAppWithMessageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfo: *mut ::core::ffi::c_void, messagefromapp: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForAppWithMessageAsync: usize,
    pub LogAuditEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IProtectionPolicyManagerStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IProtectionPolicyManagerStatics4 {
    type Vtable = IProtectionPolicyManagerStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IProtectionPolicyManagerStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20b794db_ccbd_490f_8c83_49ccb77aea6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsRoamableProtectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAccessWithBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, targetidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfo: *mut ::core::ffi::c_void, messagefromapp: ::std::mem::MaybeUninit<::windows_core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessWithBehaviorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAccessForAppWithBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceidentity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfo: *mut ::core::ffi::c_void, messagefromapp: ::std::mem::MaybeUninit<::windows_core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAccessForAppWithBehaviorAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub RequestAccessToFilesForAppAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceitemlist: *mut ::core::ffi::c_void, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    RequestAccessToFilesForAppAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub RequestAccessToFilesForAppWithMessageAndBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceitemlist: *mut ::core::ffi::c_void, apppackagefamilyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, auditinfo: *mut ::core::ffi::c_void, messagefromapp: ::std::mem::MaybeUninit<::windows_core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    RequestAccessToFilesForAppWithMessageAndBehaviorAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub RequestAccessToFilesForProcessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceitemlist: *mut ::core::ffi::c_void, processid: u32, auditinfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    RequestAccessToFilesForProcessAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub RequestAccessToFilesForProcessWithMessageAndBehaviorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceitemlist: *mut ::core::ffi::c_void, processid: u32, auditinfo: *mut ::core::ffi::c_void, messagefromapp: ::std::mem::MaybeUninit<::windows_core::HSTRING>, behavior: ProtectionPolicyRequestAccessBehavior, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    RequestAccessToFilesForProcessWithMessageAndBehaviorAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub IsFileProtectionRequiredAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    IsFileProtectionRequiredAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub IsFileProtectionRequiredForNewFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parentfolder: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, desiredname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    IsFileProtectionRequiredForNewFileAsync: usize,
    pub PrimaryManagedIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub GetPrimaryManagedIdentityForIdentity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identity: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IThreadNetworkContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IThreadNetworkContext {
    type Vtable = IThreadNetworkContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IThreadNetworkContext {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa4ea8e9_ef13_405a_b12c_d7348c6f41fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadNetworkContext_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct BufferProtectUnprotectResult(::windows_core::IUnknown);
impl BufferProtectUnprotectResult {
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Buffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProtectionInfo(&self) -> ::windows_core::Result<DataProtectionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for BufferProtectUnprotectResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.BufferProtectUnprotectResult;{47995edc-6cec-4e3a-b251-9e7485d79e7a})");
}
unsafe impl ::windows_core::Interface for BufferProtectUnprotectResult {
    type Vtable = IBufferProtectUnprotectResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BufferProtectUnprotectResult {
    const IID: ::windows_core::GUID = <IBufferProtectUnprotectResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BufferProtectUnprotectResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.BufferProtectUnprotectResult";
}
::windows_core::imp::interface_hierarchy!(BufferProtectUnprotectResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BufferProtectUnprotectResult {}
unsafe impl ::core::marker::Sync for BufferProtectUnprotectResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct DataProtectionInfo(::windows_core::IUnknown);
impl DataProtectionInfo {
    pub fn Status(&self) -> ::windows_core::Result<DataProtectionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Identity(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Identity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for DataProtectionInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.DataProtectionInfo;{8420b0c1-5e31-4405-9540-3f943af0cb26})");
}
unsafe impl ::windows_core::Interface for DataProtectionInfo {
    type Vtable = IDataProtectionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DataProtectionInfo {
    const IID: ::windows_core::GUID = <IDataProtectionInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DataProtectionInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.DataProtectionInfo";
}
::windows_core::imp::interface_hierarchy!(DataProtectionInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DataProtectionInfo {}
unsafe impl ::core::marker::Sync for DataProtectionInfo {}
pub struct DataProtectionManager;
impl DataProtectionManager {
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectAsync<P0>(data: P0, identity: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectAsync)(::windows_core::Interface::as_raw(this), data.try_into_param()?.abi(), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectAsync<P0>(data: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BufferProtectUnprotectResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnprotectAsync)(::windows_core::Interface::as_raw(this), data.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ProtectStreamAsync<P0, P1>(unprotectedstream: P0, identity: &::windows_core::HSTRING, protectedstream: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IInputStream>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IOutputStream>,
    {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectStreamAsync)(::windows_core::Interface::as_raw(this), unprotectedstream.try_into_param()?.abi(), ::core::mem::transmute_copy(identity), protectedstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn UnprotectStreamAsync<P0, P1>(protectedstream: P0, unprotectedstream: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IInputStream>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IOutputStream>,
    {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnprotectStreamAsync)(::windows_core::Interface::as_raw(this), protectedstream.try_into_param()?.abi(), unprotectedstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetProtectionInfoAsync<P0>(protecteddata: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetProtectionInfoAsync)(::windows_core::Interface::as_raw(this), protecteddata.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetStreamProtectionInfoAsync<P0>(protectedstream: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<DataProtectionInfo>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IInputStream>,
    {
        Self::IDataProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStreamProtectionInfoAsync)(::windows_core::Interface::as_raw(this), protectedstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDataProtectionManagerStatics<R, F: FnOnce(&IDataProtectionManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DataProtectionManager, IDataProtectionManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for DataProtectionManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.DataProtectionManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FileProtectionInfo(::windows_core::IUnknown);
impl FileProtectionInfo {
    pub fn Status(&self) -> ::windows_core::Result<FileProtectionStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsRoamable(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRoamable)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Identity(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Identity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsProtectWhileOpenSupported(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFileProtectionInfo2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsProtectWhileOpenSupported)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for FileProtectionInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.FileProtectionInfo;{4ee96486-147e-4dd0-8faf-5253ed91ad0c})");
}
unsafe impl ::windows_core::Interface for FileProtectionInfo {
    type Vtable = IFileProtectionInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileProtectionInfo {
    const IID: ::windows_core::GUID = <IFileProtectionInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileProtectionInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileProtectionInfo";
}
::windows_core::imp::interface_hierarchy!(FileProtectionInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for FileProtectionInfo {}
unsafe impl ::core::marker::Sync for FileProtectionInfo {}
pub struct FileProtectionManager;
impl FileProtectionManager {
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn ProtectAsync<P0>(target: P0, identity: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
    {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectAsync)(::windows_core::Interface::as_raw(this), target.try_into_param()?.abi(), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CopyProtectionAsync<P0, P1>(source: P0, target: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
    {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CopyProtectionAsync)(::windows_core::Interface::as_raw(this), source.try_into_param()?.abi(), target.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetProtectionInfoAsync<P0>(source: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
    {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetProtectionInfoAsync)(::windows_core::Interface::as_raw(this), source.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn SaveFileAsContainerAsync<P0>(protectedfile: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveFileAsContainerAsync)(::windows_core::Interface::as_raw(this), protectedfile.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFileFromContainerAsync<P0>(containerfile: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFileFromContainerAsync)(::windows_core::Interface::as_raw(this), containerfile.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFileFromContainerWithTargetAsync<P0, P1>(containerfile: P0, target: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
    {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFileFromContainerWithTargetAsync)(::windows_core::Interface::as_raw(this), containerfile.try_into_param()?.abi(), target.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn CreateProtectedAndOpenAsync<P0>(parentfolder: P0, desiredname: &::windows_core::HSTRING, identity: &::windows_core::HSTRING, collisionoption: super::super::Storage::CreationCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectedFileCreateResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFolder>,
    {
        Self::IFileProtectionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateProtectedAndOpenAsync)(::windows_core::Interface::as_raw(this), parentfolder.try_into_param()?.abi(), ::core::mem::transmute_copy(desiredname), ::core::mem::transmute_copy(identity), collisionoption, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn IsContainerAsync<P0>(file: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsContainerAsync)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn LoadFileFromContainerWithTargetAndNameCollisionOptionAsync<P0, P1>(containerfile: P0, target: P1, collisionoption: super::super::Storage::NameCollisionOption) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerImportResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
    {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LoadFileFromContainerWithTargetAndNameCollisionOptionAsync)(::windows_core::Interface::as_raw(this), containerfile.try_into_param()?.abi(), target.try_into_param()?.abi(), collisionoption, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn SaveFileAsContainerWithSharingAsync<P0, P1>(protectedfile: P0, sharedwithidentities: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectedContainerExportResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IFileProtectionManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SaveFileAsContainerWithSharingAsync)(::windows_core::Interface::as_raw(this), protectedfile.try_into_param()?.abi(), sharedwithidentities.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn UnprotectAsync<P0>(target: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
    {
        Self::IFileProtectionManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnprotectAsync)(::windows_core::Interface::as_raw(this), target.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn UnprotectWithOptionsAsync<P0, P1>(target: P0, options: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<FileProtectionInfo>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
        P1: ::windows_core::IntoParam<FileUnprotectOptions>,
    {
        Self::IFileProtectionManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UnprotectWithOptionsAsync)(::windows_core::Interface::as_raw(this), target.try_into_param()?.abi(), options.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileProtectionManagerStatics<R, F: FnOnce(&IFileProtectionManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileProtectionManager, IFileProtectionManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IFileProtectionManagerStatics2<R, F: FnOnce(&IFileProtectionManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileProtectionManager, IFileProtectionManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IFileProtectionManagerStatics3<R, F: FnOnce(&IFileProtectionManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileProtectionManager, IFileProtectionManagerStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for FileProtectionManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileProtectionManager";
}
#[doc = "Required features: `\"deprecated\"`"]
#[cfg(feature = "deprecated")]
pub struct FileRevocationManager;
#[cfg(feature = "deprecated")]
impl FileRevocationManager {
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub fn ProtectAsync<P0>(storageitem: P0, enterpriseidentity: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
    {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectAsync)(::windows_core::Interface::as_raw(this), storageitem.try_into_param()?.abi(), ::core::mem::transmute_copy(enterpriseidentity), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub fn CopyProtectionAsync<P0, P1>(sourcestorageitem: P0, targetstorageitem: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
    {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CopyProtectionAsync)(::windows_core::Interface::as_raw(this), sourcestorageitem.try_into_param()?.abi(), targetstorageitem.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"deprecated\"`"]
    #[cfg(feature = "deprecated")]
    pub fn Revoke(enterpriseidentity: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IFileRevocationManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).Revoke)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(enterpriseidentity)).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`, `\"deprecated\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage", feature = "deprecated"))]
    pub fn GetStatusAsync<P0>(storageitem: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<FileProtectionStatus>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
    {
        Self::IFileRevocationManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetStatusAsync)(::windows_core::Interface::as_raw(this), storageitem.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    #[cfg(feature = "deprecated")]
    pub fn IFileRevocationManagerStatics<R, F: FnOnce(&IFileRevocationManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileRevocationManager, IFileRevocationManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "deprecated")]
impl ::windows_core::RuntimeName for FileRevocationManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileRevocationManager";
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct FileUnprotectOptions(::windows_core::IUnknown);
impl FileUnprotectOptions {
    pub fn SetAudit(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAudit)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Audit(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Audit)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(audit: bool) -> ::windows_core::Result<FileUnprotectOptions> {
        Self::IFileUnprotectOptionsFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), audit, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileUnprotectOptionsFactory<R, F: FnOnce(&IFileUnprotectOptionsFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FileUnprotectOptions, IFileUnprotectOptionsFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for FileUnprotectOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.FileUnprotectOptions;{7d1312f1-3b0d-4dd8-a1f8-1ec53822e2f3})");
}
unsafe impl ::windows_core::Interface for FileUnprotectOptions {
    type Vtable = IFileUnprotectOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FileUnprotectOptions {
    const IID: ::windows_core::GUID = <IFileUnprotectOptions as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FileUnprotectOptions {
    const NAME: &'static str = "Windows.Security.EnterpriseData.FileUnprotectOptions";
}
::windows_core::imp::interface_hierarchy!(FileUnprotectOptions, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for FileUnprotectOptions {}
unsafe impl ::core::marker::Sync for FileUnprotectOptions {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProtectedAccessResumedEventArgs(::windows_core::IUnknown);
impl ProtectedAccessResumedEventArgs {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Identities(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Identities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProtectedAccessResumedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedAccessResumedEventArgs;{ac4dca59-5d80-4e95-8c5f-8539450eebe0})");
}
unsafe impl ::windows_core::Interface for ProtectedAccessResumedEventArgs {
    type Vtable = IProtectedAccessResumedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProtectedAccessResumedEventArgs {
    const IID: ::windows_core::GUID = <IProtectedAccessResumedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProtectedAccessResumedEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedAccessResumedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ProtectedAccessResumedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProtectedAccessResumedEventArgs {}
unsafe impl ::core::marker::Sync for ProtectedAccessResumedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProtectedAccessSuspendingEventArgs(::windows_core::IUnknown);
impl ProtectedAccessSuspendingEventArgs {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Identities(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Identities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Deadline(&self) -> ::windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Deadline)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProtectedAccessSuspendingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedAccessSuspendingEventArgs;{75a193e0-a344-429f-b975-04fc1f88c185})");
}
unsafe impl ::windows_core::Interface for ProtectedAccessSuspendingEventArgs {
    type Vtable = IProtectedAccessSuspendingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProtectedAccessSuspendingEventArgs {
    const IID: ::windows_core::GUID = <IProtectedAccessSuspendingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProtectedAccessSuspendingEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedAccessSuspendingEventArgs";
}
::windows_core::imp::interface_hierarchy!(ProtectedAccessSuspendingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProtectedAccessSuspendingEventArgs {}
unsafe impl ::core::marker::Sync for ProtectedAccessSuspendingEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProtectedContainerExportResult(::windows_core::IUnknown);
impl ProtectedContainerExportResult {
    pub fn Status(&self) -> ::windows_core::Result<ProtectedImportExportStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage\"`"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProtectedContainerExportResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedContainerExportResult;{3948ef95-f7fb-4b42-afb0-df70b41543c1})");
}
unsafe impl ::windows_core::Interface for ProtectedContainerExportResult {
    type Vtable = IProtectedContainerExportResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProtectedContainerExportResult {
    const IID: ::windows_core::GUID = <IProtectedContainerExportResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProtectedContainerExportResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContainerExportResult";
}
::windows_core::imp::interface_hierarchy!(ProtectedContainerExportResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProtectedContainerExportResult {}
unsafe impl ::core::marker::Sync for ProtectedContainerExportResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProtectedContainerImportResult(::windows_core::IUnknown);
impl ProtectedContainerImportResult {
    pub fn Status(&self) -> ::windows_core::Result<ProtectedImportExportStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage\"`"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProtectedContainerImportResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedContainerImportResult;{cdb780d1-e7bb-4d1a-9339-34dc41149f9b})");
}
unsafe impl ::windows_core::Interface for ProtectedContainerImportResult {
    type Vtable = IProtectedContainerImportResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProtectedContainerImportResult {
    const IID: ::windows_core::GUID = <IProtectedContainerImportResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProtectedContainerImportResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContainerImportResult";
}
::windows_core::imp::interface_hierarchy!(ProtectedContainerImportResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProtectedContainerImportResult {}
unsafe impl ::core::marker::Sync for ProtectedContainerImportResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProtectedContentRevokedEventArgs(::windows_core::IUnknown);
impl ProtectedContentRevokedEventArgs {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Identities(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Identities)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProtectedContentRevokedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedContentRevokedEventArgs;{63686821-58b9-47ee-93d9-f0f741cf43f0})");
}
unsafe impl ::windows_core::Interface for ProtectedContentRevokedEventArgs {
    type Vtable = IProtectedContentRevokedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProtectedContentRevokedEventArgs {
    const IID: ::windows_core::GUID = <IProtectedContentRevokedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProtectedContentRevokedEventArgs {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedContentRevokedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ProtectedContentRevokedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProtectedContentRevokedEventArgs {}
unsafe impl ::core::marker::Sync for ProtectedContentRevokedEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProtectedFileCreateResult(::windows_core::IUnknown);
impl ProtectedFileCreateResult {
    #[doc = "Required features: `\"Storage\"`"]
    #[cfg(feature = "Storage")]
    pub fn File(&self) -> ::windows_core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).File)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Storage_Streams\"`"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ProtectionInfo(&self) -> ::windows_core::Result<FileProtectionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectionInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ProtectedFileCreateResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectedFileCreateResult;{28e3ed6a-e9e7-4a03-9f53-bdb16172699b})");
}
unsafe impl ::windows_core::Interface for ProtectedFileCreateResult {
    type Vtable = IProtectedFileCreateResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProtectedFileCreateResult {
    const IID: ::windows_core::GUID = <IProtectedFileCreateResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProtectedFileCreateResult {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectedFileCreateResult";
}
::windows_core::imp::interface_hierarchy!(ProtectedFileCreateResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProtectedFileCreateResult {}
unsafe impl ::core::marker::Sync for ProtectedFileCreateResult {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProtectionPolicyAuditInfo(::windows_core::IUnknown);
impl ProtectionPolicyAuditInfo {
    pub fn SetAction(&self, value: ProtectionPolicyAuditAction) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAction)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Action(&self) -> ::windows_core::Result<ProtectionPolicyAuditAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Action)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDataDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDataDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DataDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DataDescription)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSourceDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSourceDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SourceDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceDescription)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTargetDescription(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTargetDescription)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TargetDescription(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TargetDescription)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(action: ProtectionPolicyAuditAction, datadescription: &::windows_core::HSTRING, sourcedescription: &::windows_core::HSTRING, targetdescription: &::windows_core::HSTRING) -> ::windows_core::Result<ProtectionPolicyAuditInfo> {
        Self::IProtectionPolicyAuditInfoFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), action, ::core::mem::transmute_copy(datadescription), ::core::mem::transmute_copy(sourcedescription), ::core::mem::transmute_copy(targetdescription), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithActionAndDataDescription(action: ProtectionPolicyAuditAction, datadescription: &::windows_core::HSTRING) -> ::windows_core::Result<ProtectionPolicyAuditInfo> {
        Self::IProtectionPolicyAuditInfoFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithActionAndDataDescription)(::windows_core::Interface::as_raw(this), action, ::core::mem::transmute_copy(datadescription), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProtectionPolicyAuditInfoFactory<R, F: FnOnce(&IProtectionPolicyAuditInfoFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ProtectionPolicyAuditInfo, IProtectionPolicyAuditInfoFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ProtectionPolicyAuditInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectionPolicyAuditInfo;{425ab7e4-feb7-44fc-b3bb-c3c4d7ecbebb})");
}
unsafe impl ::windows_core::Interface for ProtectionPolicyAuditInfo {
    type Vtable = IProtectionPolicyAuditInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProtectionPolicyAuditInfo {
    const IID: ::windows_core::GUID = <IProtectionPolicyAuditInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProtectionPolicyAuditInfo {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectionPolicyAuditInfo";
}
::windows_core::imp::interface_hierarchy!(ProtectionPolicyAuditInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProtectionPolicyAuditInfo {}
unsafe impl ::core::marker::Sync for ProtectionPolicyAuditInfo {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ProtectionPolicyManager(::windows_core::IUnknown);
impl ProtectionPolicyManager {
    pub fn SetIdentity(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIdentity)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Identity(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Identity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetShowEnterpriseIndicator(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IProtectionPolicyManager2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetShowEnterpriseIndicator)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShowEnterpriseIndicator(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IProtectionPolicyManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ShowEnterpriseIndicator)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsIdentityManaged(identity: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsIdentityManaged)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        })
    }
    pub fn TryApplyProcessUIPolicy(identity: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TryApplyProcessUIPolicy)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        })
    }
    pub fn ClearProcessUIPolicy() -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).ClearProcessUIPolicy)(::windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn CreateCurrentThreadNetworkContext(identity: &::windows_core::HSTRING) -> ::windows_core::Result<ThreadNetworkContext> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCurrentThreadNetworkContext)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Networking\"`"]
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub fn GetPrimaryManagedIdentityForNetworkEndpointAsync<P0>(endpointhost: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<::windows_core::HSTRING>>
    where
        P0: ::windows_core::IntoParam<super::super::Networking::HostName>,
    {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPrimaryManagedIdentityForNetworkEndpointAsync)(::windows_core::Interface::as_raw(this), endpointhost.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn RevokeContent(identity: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RevokeContent)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identity)).ok() })
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<ProtectionPolicyManager> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ProtectedAccessSuspending<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::EventHandler<ProtectedAccessSuspendingEventArgs>>,
    {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectedAccessSuspending)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProtectedAccessSuspending(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveProtectedAccessSuspending)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ProtectedAccessResumed<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::EventHandler<ProtectedAccessResumedEventArgs>>,
    {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectedAccessResumed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProtectedAccessResumed(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveProtectedAccessResumed)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn ProtectedContentRevoked<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::EventHandler<ProtectedContentRevokedEventArgs>>,
    {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ProtectedContentRevoked)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProtectedContentRevoked(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe { (::windows_core::Interface::vtable(this).RemoveProtectedContentRevoked)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn CheckAccess(sourceidentity: &::windows_core::HSTRING, targetidentity: &::windows_core::HSTRING) -> ::windows_core::Result<ProtectionPolicyEvaluationResult> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CheckAccess)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(targetidentity), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync(sourceidentity: &::windows_core::HSTRING, targetidentity: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(targetidentity), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn HasContentBeenRevokedSince(identity: &::windows_core::HSTRING, since: super::super::Foundation::DateTime) -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasContentBeenRevokedSince)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identity), since, &mut result__).from_abi(result__)
        })
    }
    pub fn CheckAccessForApp(sourceidentity: &::windows_core::HSTRING, apppackagefamilyname: &::windows_core::HSTRING) -> ::windows_core::Result<ProtectionPolicyEvaluationResult> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CheckAccessForApp)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(apppackagefamilyname), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForAppAsync(sourceidentity: &::windows_core::HSTRING, apppackagefamilyname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessForAppAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(apppackagefamilyname), &mut result__).from_abi(result__)
        })
    }
    pub fn GetEnforcementLevel(identity: &::windows_core::HSTRING) -> ::windows_core::Result<EnforcementLevel> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEnforcementLevel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        })
    }
    pub fn IsUserDecryptionAllowed(identity: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUserDecryptionAllowed)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        })
    }
    pub fn IsProtectionUnderLockRequired(identity: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsProtectionUnderLockRequired)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn PolicyChanged<P0>(handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::EventHandler<::windows_core::IInspectable>>,
    {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PolicyChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePolicyChanged(token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).RemovePolicyChanged)(::windows_core::Interface::as_raw(this), token).ok() })
    }
    pub fn IsProtectionEnabled() -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsProtectionEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessWithAuditingInfoAsync<P0>(sourceidentity: &::windows_core::HSTRING, targetidentity: &::windows_core::HSTRING, auditinfo: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>
    where
        P0: ::windows_core::IntoParam<ProtectionPolicyAuditInfo>,
    {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessWithAuditingInfoAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(targetidentity), auditinfo.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessWithMessageAsync<P0>(sourceidentity: &::windows_core::HSTRING, targetidentity: &::windows_core::HSTRING, auditinfo: P0, messagefromapp: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>
    where
        P0: ::windows_core::IntoParam<ProtectionPolicyAuditInfo>,
    {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessWithMessageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(targetidentity), auditinfo.into_param().abi(), ::core::mem::transmute_copy(messagefromapp), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForAppWithAuditingInfoAsync<P0>(sourceidentity: &::windows_core::HSTRING, apppackagefamilyname: &::windows_core::HSTRING, auditinfo: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>
    where
        P0: ::windows_core::IntoParam<ProtectionPolicyAuditInfo>,
    {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessForAppWithAuditingInfoAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(apppackagefamilyname), auditinfo.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForAppWithMessageAsync<P0>(sourceidentity: &::windows_core::HSTRING, apppackagefamilyname: &::windows_core::HSTRING, auditinfo: P0, messagefromapp: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>
    where
        P0: ::windows_core::IntoParam<ProtectionPolicyAuditInfo>,
    {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessForAppWithMessageAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(apppackagefamilyname), auditinfo.into_param().abi(), ::core::mem::transmute_copy(messagefromapp), &mut result__).from_abi(result__)
        })
    }
    pub fn LogAuditEvent<P0>(sourceidentity: &::windows_core::HSTRING, targetidentity: &::windows_core::HSTRING, auditinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<ProtectionPolicyAuditInfo>,
    {
        Self::IProtectionPolicyManagerStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).LogAuditEvent)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(targetidentity), auditinfo.into_param().abi()).ok() })
    }
    pub fn IsRoamableProtectionEnabled(identity: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRoamableProtectionEnabled)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessWithBehaviorAsync<P0>(sourceidentity: &::windows_core::HSTRING, targetidentity: &::windows_core::HSTRING, auditinfo: P0, messagefromapp: &::windows_core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>
    where
        P0: ::windows_core::IntoParam<ProtectionPolicyAuditInfo>,
    {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessWithBehaviorAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(targetidentity), auditinfo.into_param().abi(), ::core::mem::transmute_copy(messagefromapp), behavior, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessForAppWithBehaviorAsync<P0>(sourceidentity: &::windows_core::HSTRING, apppackagefamilyname: &::windows_core::HSTRING, auditinfo: P0, messagefromapp: &::windows_core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>
    where
        P0: ::windows_core::IntoParam<ProtectionPolicyAuditInfo>,
    {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessForAppWithBehaviorAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(sourceidentity), ::core::mem::transmute_copy(apppackagefamilyname), auditinfo.into_param().abi(), ::core::mem::transmute_copy(messagefromapp), behavior, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn RequestAccessToFilesForAppAsync<P0, P1>(sourceitemlist: P0, apppackagefamilyname: &::windows_core::HSTRING, auditinfo: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>,
        P1: ::windows_core::IntoParam<ProtectionPolicyAuditInfo>,
    {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessToFilesForAppAsync)(::windows_core::Interface::as_raw(this), sourceitemlist.try_into_param()?.abi(), ::core::mem::transmute_copy(apppackagefamilyname), auditinfo.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn RequestAccessToFilesForAppWithMessageAndBehaviorAsync<P0, P1>(sourceitemlist: P0, apppackagefamilyname: &::windows_core::HSTRING, auditinfo: P1, messagefromapp: &::windows_core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>,
        P1: ::windows_core::IntoParam<ProtectionPolicyAuditInfo>,
    {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessToFilesForAppWithMessageAndBehaviorAsync)(::windows_core::Interface::as_raw(this), sourceitemlist.try_into_param()?.abi(), ::core::mem::transmute_copy(apppackagefamilyname), auditinfo.into_param().abi(), ::core::mem::transmute_copy(messagefromapp), behavior, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn RequestAccessToFilesForProcessAsync<P0, P1>(sourceitemlist: P0, processid: u32, auditinfo: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>,
        P1: ::windows_core::IntoParam<ProtectionPolicyAuditInfo>,
    {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessToFilesForProcessAsync)(::windows_core::Interface::as_raw(this), sourceitemlist.try_into_param()?.abi(), processid, auditinfo.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn RequestAccessToFilesForProcessWithMessageAndBehaviorAsync<P0, P1>(sourceitemlist: P0, processid: u32, auditinfo: P1, messagefromapp: &::windows_core::HSTRING, behavior: ProtectionPolicyRequestAccessBehavior) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>,
        P1: ::windows_core::IntoParam<ProtectionPolicyAuditInfo>,
    {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAccessToFilesForProcessWithMessageAndBehaviorAsync)(::windows_core::Interface::as_raw(this), sourceitemlist.try_into_param()?.abi(), processid, auditinfo.into_param().abi(), ::core::mem::transmute_copy(messagefromapp), behavior, &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn IsFileProtectionRequiredAsync<P0>(target: P0, identity: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageItem>,
    {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFileProtectionRequiredAsync)(::windows_core::Interface::as_raw(this), target.try_into_param()?.abi(), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        })
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn IsFileProtectionRequiredForNewFileAsync<P0>(parentfolder: P0, identity: &::windows_core::HSTRING, desiredname: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFolder>,
    {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsFileProtectionRequiredForNewFileAsync)(::windows_core::Interface::as_raw(this), parentfolder.try_into_param()?.abi(), ::core::mem::transmute_copy(identity), ::core::mem::transmute_copy(desiredname), &mut result__).from_abi(result__)
        })
    }
    pub fn PrimaryManagedIdentity() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PrimaryManagedIdentity)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GetPrimaryManagedIdentityForIdentity(identity: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::IProtectionPolicyManagerStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPrimaryManagedIdentityForIdentity)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(identity), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IProtectionPolicyManagerStatics<R, F: FnOnce(&IProtectionPolicyManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IProtectionPolicyManagerStatics2<R, F: FnOnce(&IProtectionPolicyManagerStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IProtectionPolicyManagerStatics3<R, F: FnOnce(&IProtectionPolicyManagerStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IProtectionPolicyManagerStatics4<R, F: FnOnce(&IProtectionPolicyManagerStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ProtectionPolicyManager, IProtectionPolicyManagerStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ProtectionPolicyManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ProtectionPolicyManager;{d5703e18-a08d-47e6-a240-9934d7165eb5})");
}
unsafe impl ::windows_core::Interface for ProtectionPolicyManager {
    type Vtable = IProtectionPolicyManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ProtectionPolicyManager {
    const IID: ::windows_core::GUID = <IProtectionPolicyManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ProtectionPolicyManager {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ProtectionPolicyManager";
}
::windows_core::imp::interface_hierarchy!(ProtectionPolicyManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ProtectionPolicyManager {}
unsafe impl ::core::marker::Sync for ProtectionPolicyManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ThreadNetworkContext(::windows_core::IUnknown);
impl ThreadNetworkContext {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for ThreadNetworkContext {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Security.EnterpriseData.ThreadNetworkContext;{fa4ea8e9-ef13-405a-b12c-d7348c6f41fc})");
}
unsafe impl ::windows_core::Interface for ThreadNetworkContext {
    type Vtable = IThreadNetworkContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ThreadNetworkContext {
    const IID: ::windows_core::GUID = <IThreadNetworkContext as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ThreadNetworkContext {
    const NAME: &'static str = "Windows.Security.EnterpriseData.ThreadNetworkContext";
}
::windows_core::imp::interface_hierarchy!(ThreadNetworkContext, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for ThreadNetworkContext {}
unsafe impl ::core::marker::Send for ThreadNetworkContext {}
unsafe impl ::core::marker::Sync for ThreadNetworkContext {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DataProtectionStatus(pub i32);
impl DataProtectionStatus {
    pub const ProtectedToOtherIdentity: Self = Self(0i32);
    pub const Protected: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Unprotected: Self = Self(3i32);
    pub const LicenseExpired: Self = Self(4i32);
    pub const AccessSuspended: Self = Self(5i32);
}
impl ::core::marker::Copy for DataProtectionStatus {}
impl ::core::clone::Clone for DataProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DataProtectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for DataProtectionStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DataProtectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataProtectionStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DataProtectionStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.DataProtectionStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EnforcementLevel(pub i32);
impl EnforcementLevel {
    pub const NoProtection: Self = Self(0i32);
    pub const Silent: Self = Self(1i32);
    pub const Override: Self = Self(2i32);
    pub const Block: Self = Self(3i32);
}
impl ::core::marker::Copy for EnforcementLevel {}
impl ::core::clone::Clone for EnforcementLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EnforcementLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EnforcementLevel {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EnforcementLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnforcementLevel").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for EnforcementLevel {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.EnforcementLevel;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FileProtectionStatus(pub i32);
impl FileProtectionStatus {
    pub const Undetermined: Self = Self(0i32);
    pub const Unknown: Self = Self(0i32);
    pub const Unprotected: Self = Self(1i32);
    pub const Revoked: Self = Self(2i32);
    pub const Protected: Self = Self(3i32);
    pub const ProtectedByOtherUser: Self = Self(4i32);
    pub const ProtectedToOtherEnterprise: Self = Self(5i32);
    pub const NotProtectable: Self = Self(6i32);
    pub const ProtectedToOtherIdentity: Self = Self(7i32);
    pub const LicenseExpired: Self = Self(8i32);
    pub const AccessSuspended: Self = Self(9i32);
    pub const FileInUse: Self = Self(10i32);
}
impl ::core::marker::Copy for FileProtectionStatus {}
impl ::core::clone::Clone for FileProtectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileProtectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FileProtectionStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FileProtectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileProtectionStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FileProtectionStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.FileProtectionStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProtectedImportExportStatus(pub i32);
impl ProtectedImportExportStatus {
    pub const Ok: Self = Self(0i32);
    pub const Undetermined: Self = Self(1i32);
    pub const Unprotected: Self = Self(2i32);
    pub const Revoked: Self = Self(3i32);
    pub const NotRoamable: Self = Self(4i32);
    pub const ProtectedToOtherIdentity: Self = Self(5i32);
    pub const LicenseExpired: Self = Self(6i32);
    pub const AccessSuspended: Self = Self(7i32);
}
impl ::core::marker::Copy for ProtectedImportExportStatus {}
impl ::core::clone::Clone for ProtectedImportExportStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectedImportExportStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProtectedImportExportStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProtectedImportExportStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectedImportExportStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProtectedImportExportStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectedImportExportStatus;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProtectionPolicyAuditAction(pub i32);
impl ProtectionPolicyAuditAction {
    pub const Decrypt: Self = Self(0i32);
    pub const CopyToLocation: Self = Self(1i32);
    pub const SendToRecipient: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
}
impl ::core::marker::Copy for ProtectionPolicyAuditAction {}
impl ::core::clone::Clone for ProtectionPolicyAuditAction {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectionPolicyAuditAction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProtectionPolicyAuditAction {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProtectionPolicyAuditAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyAuditAction").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProtectionPolicyAuditAction {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyAuditAction;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProtectionPolicyEvaluationResult(pub i32);
impl ProtectionPolicyEvaluationResult {
    pub const Allowed: Self = Self(0i32);
    pub const Blocked: Self = Self(1i32);
    pub const ConsentRequired: Self = Self(2i32);
}
impl ::core::marker::Copy for ProtectionPolicyEvaluationResult {}
impl ::core::clone::Clone for ProtectionPolicyEvaluationResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectionPolicyEvaluationResult {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProtectionPolicyEvaluationResult {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProtectionPolicyEvaluationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyEvaluationResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProtectionPolicyEvaluationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyEvaluationResult;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ProtectionPolicyRequestAccessBehavior(pub i32);
impl ProtectionPolicyRequestAccessBehavior {
    pub const Decrypt: Self = Self(0i32);
    pub const TreatOverridePolicyAsBlock: Self = Self(1i32);
}
impl ::core::marker::Copy for ProtectionPolicyRequestAccessBehavior {}
impl ::core::clone::Clone for ProtectionPolicyRequestAccessBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ProtectionPolicyRequestAccessBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ProtectionPolicyRequestAccessBehavior {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ProtectionPolicyRequestAccessBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProtectionPolicyRequestAccessBehavior").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ProtectionPolicyRequestAccessBehavior {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Security.EnterpriseData.ProtectionPolicyRequestAccessBehavior;i4)");
}
