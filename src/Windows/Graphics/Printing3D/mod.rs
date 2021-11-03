#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrint3DManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrint3DManager {
    type Vtable = IPrint3DManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1294977802, 29542, 18801, [139, 213, 23, 196, 227, 232, 198, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrint3DManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrint3DManagerStatics {
    type Vtable = IPrint3DManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(250727166, 43437, 19464, [169, 23, 29, 31, 134, 62, 171, 203]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrint3DTask(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrint3DTask {
    type Vtable = IPrint3DTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2363740288, 8472, 19496, [128, 222, 244, 38, 215, 1, 145, 174]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DTask_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrint3DTaskCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrint3DTaskCompletedEventArgs {
    type Vtable = IPrint3DTaskCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3424195759, 9748, 20253, [172, 204, 214, 252, 79, 218, 84, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DTaskCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Print3DTaskCompletion) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Print3DTaskDetail) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrint3DTaskRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrint3DTaskRequest {
    type Vtable = IPrint3DTaskRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(630572143, 8773, 19546, [135, 49, 13, 96, 77, 198, 188, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DTaskRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, title: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, printerid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, handler: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrint3DTaskRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrint3DTaskRequestedEventArgs {
    type Vtable = IPrint3DTaskRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(353154943, 6341, 16599, [159, 64, 250, 179, 9, 110, 5, 169]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DTaskRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrint3DTaskSourceChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrint3DTaskSourceChangedEventArgs {
    type Vtable = IPrint3DTaskSourceChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1540175023, 9449, 19472, [141, 7, 20, 195, 70, 186, 63, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DTaskSourceChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrint3DTaskSourceRequestedArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrint3DTaskSourceRequestedArgs {
    type Vtable = IPrint3DTaskSourceRequestedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3346832058, 9391, 16973, [163, 191, 146, 37, 12, 53, 86, 2]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DTaskSourceRequestedArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, source: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3D3MFPackage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3D3MFPackage {
    type Vtable = IPrinting3D3MFPackage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4132296136, 10935, 17833, [161, 183, 38, 126, 148, 141, 91, 24]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3D3MFPackage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3D3MFPackage2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3D3MFPackage2 {
    type Vtable = IPrinting3D3MFPackage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2522643140, 37835, 17456, [146, 184, 120, 156, 212, 84, 248, 131]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3D3MFPackage2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Printing3DPackageCompression) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Printing3DPackageCompression) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3D3MFPackageStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3D3MFPackageStatics {
    type Vtable = IPrinting3D3MFPackageStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1884871087, 31386, 18311, [184, 23, 246, 244, 89, 33, 72, 35]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3D3MFPackageStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DBaseMaterial(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DBaseMaterial {
    type Vtable = IPrinting3DBaseMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3505448771, 50444, 19403, [157, 4, 252, 22, 173, 206, 162, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DBaseMaterial_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DBaseMaterialGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DBaseMaterialGroup {
    type Vtable = IPrinting3DBaseMaterialGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2498785464, 9493, 19085, [161, 240, 208, 252, 19, 208, 96, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DBaseMaterialGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DBaseMaterialGroupFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DBaseMaterialGroupFactory {
    type Vtable = IPrinting3DBaseMaterialGroupFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1544898268, 34455, 16787, [151, 107, 132, 187, 65, 22, 229, 191]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DBaseMaterialGroupFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, materialgroupid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DBaseMaterialStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DBaseMaterialStatics {
    type Vtable = IPrinting3DBaseMaterialStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2170177468, 14154, 18285, [190, 146, 62, 207, 209, 203, 151, 118]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DBaseMaterialStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DColorMaterial(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DColorMaterial {
    type Vtable = IPrinting3DColorMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3783891240, 31975, 17029, [163, 93, 241, 69, 201, 81, 12, 123]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DColorMaterial_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DColorMaterial2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DColorMaterial2 {
    type Vtable = IPrinting3DColorMaterial2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4205897810, 2799, 17641, [157, 221, 54, 238, 234, 90, 205, 68]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DColorMaterial2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DColorMaterialGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DColorMaterialGroup {
    type Vtable = IPrinting3DColorMaterialGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1731536, 43743, 16934, [175, 233, 243, 105, 160, 180, 80, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DColorMaterialGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DColorMaterialGroupFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DColorMaterialGroupFactory {
    type Vtable = IPrinting3DColorMaterialGroupFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1909689709, 45546, 19035, [188, 84, 25, 198, 95, 61, 240, 68]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DColorMaterialGroupFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, materialgroupid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DComponent(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DComponent {
    type Vtable = IPrinting3DComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2116581445, 49023, 19675, [162, 127, 48, 160, 20, 55, 254, 222]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DComponent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Printing3DObjectType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Printing3DObjectType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DComponentWithMatrix(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DComponentWithMatrix {
    type Vtable = IPrinting3DComponentWithMatrix_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(846852917, 3824, 17771, [154, 33, 73, 190, 190, 139, 81, 194]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DComponentWithMatrix_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DCompositeMaterial(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DCompositeMaterial {
    type Vtable = IPrinting3DCompositeMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1176647901, 22062, 20332, [136, 45, 244, 216, 65, 253, 99, 199]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DCompositeMaterial_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DCompositeMaterialGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DCompositeMaterialGroup {
    type Vtable = IPrinting3DCompositeMaterialGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2375314011, 16625, 18797, [165, 251, 52, 10, 90, 103, 142, 48]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DCompositeMaterialGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DCompositeMaterialGroup2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DCompositeMaterialGroup2 {
    type Vtable = IPrinting3DCompositeMaterialGroup2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(115895650, 32059, 16865, [148, 76, 186, 253, 228, 85, 84, 131]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DCompositeMaterialGroup2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DCompositeMaterialGroupFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DCompositeMaterialGroupFactory {
    type Vtable = IPrinting3DCompositeMaterialGroupFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3499019539, 37631, 17322, [166, 39, 141, 67, 194, 44, 129, 126]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DCompositeMaterialGroupFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, materialgroupid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DFaceReductionOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DFaceReductionOptions {
    type Vtable = IPrinting3DFaceReductionOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3154039703, 11636, 18167, [190, 133, 153, 166, 123, 187, 102, 41]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DFaceReductionOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DMaterial(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DMaterial {
    type Vtable = IPrinting3DMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(932033110, 60770, 18770, [184, 91, 3, 86, 125, 124, 70, 94]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DMaterial_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DMesh(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DMesh {
    type Vtable = IPrinting3DMesh_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(422482140, 552, 11777, [188, 32, 197, 41, 12, 191, 50, 196]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DMesh_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Printing3DBufferDescription) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Printing3DBufferDescription) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Printing3DBufferDescription) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Printing3DBufferDescription) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Printing3DBufferDescription) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Printing3DBufferDescription) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Printing3DBufferDescription) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Printing3DBufferDescription) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Printing3DMeshVerificationMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DMeshVerificationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DMeshVerificationResult {
    type Vtable = IPrinting3DMeshVerificationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(425095610, 59706, 20106, [164, 111, 222, 168, 232, 82, 25, 126]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DMeshVerificationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DModel(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DModel {
    type Vtable = IPrinting3DModel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(755052272, 21243, 37274, [119, 176, 75, 26, 59, 128, 50, 79]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DModel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Printing3DModelUnit) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Printing3DModelUnit) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DModel2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DModel2 {
    type Vtable = IPrinting3DModel2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3374344647, 51265, 18419, [168, 78, 161, 73, 253, 8, 182, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DModel2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxwaittime: super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printing3dfacereductionoptions: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printing3dfacereductionoptions: ::windows::runtime::RawPtr, maxwait: super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DModelTexture(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DModelTexture {
    type Vtable = IPrinting3DModelTexture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1571802881, 46493, 18492, [151, 187, 164, 213, 70, 209, 199, 92]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DModelTexture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Printing3DTextureEdgeBehavior) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Printing3DTextureEdgeBehavior) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Printing3DTextureEdgeBehavior) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Printing3DTextureEdgeBehavior) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DMultiplePropertyMaterial(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DMultiplePropertyMaterial {
    type Vtable = IPrinting3DMultiplePropertyMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(631645515, 50921, 18509, [162, 20, 162, 94, 87, 118, 186, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DMultiplePropertyMaterial_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DMultiplePropertyMaterialGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DMultiplePropertyMaterialGroup {
    type Vtable = IPrinting3DMultiplePropertyMaterialGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4036298009, 44729, 17685, [163, 155, 160, 136, 251, 187, 39, 124]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DMultiplePropertyMaterialGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DMultiplePropertyMaterialGroupFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DMultiplePropertyMaterialGroupFactory {
    type Vtable = IPrinting3DMultiplePropertyMaterialGroupFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(842930542, 54470, 17694, [168, 20, 77, 120, 162, 16, 254, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DMultiplePropertyMaterialGroupFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, materialgroupid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DTexture2CoordMaterial(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DTexture2CoordMaterial {
    type Vtable = IPrinting3DTexture2CoordMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2374257659, 2025, 18822, [152, 51, 141, 211, 212, 140, 104, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DTexture2CoordMaterial_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DTexture2CoordMaterialGroup(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DTexture2CoordMaterialGroup {
    type Vtable = IPrinting3DTexture2CoordMaterialGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1652391079, 28048, 20409, [159, 196, 159, 239, 243, 223, 168, 146]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DTexture2CoordMaterialGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DTexture2CoordMaterialGroup2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DTexture2CoordMaterialGroup2 {
    type Vtable = IPrinting3DTexture2CoordMaterialGroup2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1778113466, 45358, 17051, [131, 134, 223, 82, 132, 246, 232, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DTexture2CoordMaterialGroup2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DTexture2CoordMaterialGroupFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DTexture2CoordMaterialGroupFactory {
    type Vtable = IPrinting3DTexture2CoordMaterialGroupFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3417328048, 18058, 19567, [178, 162, 142, 184, 186, 141, 234, 72]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DTexture2CoordMaterialGroupFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, materialgroupid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrinting3DTextureResource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrinting3DTextureResource {
    type Vtable = IPrinting3DTextureResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2802709293, 27313, 17582, [188, 69, 162, 115, 130, 192, 211, 140]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DTextureResource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Print3DManager(::windows::runtime::IInspectable);
impl Print3DManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn TaskRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<Print3DManager, Print3DTaskRequestedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn RemoveTaskRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn GetForCurrentView() -> ::windows::runtime::Result<Print3DManager> {
        Self::IPrint3DManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Print3DManager>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn ShowPrintUIAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPrint3DManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    pub fn IPrint3DManagerStatics<R, F: FnOnce(&IPrint3DManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Print3DManager, IPrint3DManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Print3DManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DManager;{4d2fcb0a-7366-4971-8bd5-17c4e3e8c6c0})");
}
unsafe impl ::windows::runtime::Interface for Print3DManager {
    type Vtable = IPrint3DManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1294977802, 29542, 18801, [139, 213, 23, 196, 227, 232, 198, 192]);
}
impl ::windows::runtime::RuntimeName for Print3DManager {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DManager";
}
impl ::std::convert::From<Print3DManager> for ::windows::runtime::IUnknown {
    fn from(value: Print3DManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Print3DManager> for ::windows::runtime::IUnknown {
    fn from(value: &Print3DManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Print3DManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Print3DManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Print3DManager> for ::windows::runtime::IInspectable {
    fn from(value: Print3DManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Print3DManager> for ::windows::runtime::IInspectable {
    fn from(value: &Print3DManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Print3DManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Print3DManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Print3DManager {}
unsafe impl ::std::marker::Sync for Print3DManager {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Print3DTask(::windows::runtime::IInspectable);
impl Print3DTask {
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<Printing3D3MFPackage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3D3MFPackage>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn Submitting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<Print3DTask, ::windows::runtime::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn RemoveSubmitting<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn Completed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskCompletedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn RemoveCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn SourceChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskSourceChangedEventArgs>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn RemoveSourceChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Print3DTask {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DTask;{8ce3d080-2118-4c28-80de-f426d70191ae})");
}
unsafe impl ::windows::runtime::Interface for Print3DTask {
    type Vtable = IPrint3DTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2363740288, 8472, 19496, [128, 222, 244, 38, 215, 1, 145, 174]);
}
impl ::windows::runtime::RuntimeName for Print3DTask {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTask";
}
impl ::std::convert::From<Print3DTask> for ::windows::runtime::IUnknown {
    fn from(value: Print3DTask) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Print3DTask> for ::windows::runtime::IUnknown {
    fn from(value: &Print3DTask) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Print3DTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Print3DTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Print3DTask> for ::windows::runtime::IInspectable {
    fn from(value: Print3DTask) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Print3DTask> for ::windows::runtime::IInspectable {
    fn from(value: &Print3DTask) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Print3DTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Print3DTask {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Print3DTask {}
unsafe impl ::std::marker::Sync for Print3DTask {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Print3DTaskCompletedEventArgs(::windows::runtime::IInspectable);
impl Print3DTaskCompletedEventArgs {
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Completion(&self) -> ::windows::runtime::Result<Print3DTaskCompletion> {
        let this = self;
        unsafe {
            let mut result__: Print3DTaskCompletion = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Print3DTaskCompletion>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn ExtendedStatus(&self) -> ::windows::runtime::Result<Print3DTaskDetail> {
        let this = self;
        unsafe {
            let mut result__: Print3DTaskDetail = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Print3DTaskDetail>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Print3DTaskCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DTaskCompletedEventArgs;{cc1914af-2614-4f1d-accc-d6fc4fda5455})");
}
unsafe impl ::windows::runtime::Interface for Print3DTaskCompletedEventArgs {
    type Vtable = IPrint3DTaskCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3424195759, 9748, 20253, [172, 204, 214, 252, 79, 218, 84, 85]);
}
impl ::windows::runtime::RuntimeName for Print3DTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskCompletedEventArgs";
}
impl ::std::convert::From<Print3DTaskCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: Print3DTaskCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Print3DTaskCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &Print3DTaskCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Print3DTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Print3DTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Print3DTaskCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: Print3DTaskCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Print3DTaskCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &Print3DTaskCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Print3DTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Print3DTaskCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Print3DTaskCompletedEventArgs {}
unsafe impl ::std::marker::Sync for Print3DTaskCompletedEventArgs {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct Print3DTaskCompletion(pub i32);
impl Print3DTaskCompletion {
    pub const Abandoned: Print3DTaskCompletion = Print3DTaskCompletion(0i32);
    pub const Canceled: Print3DTaskCompletion = Print3DTaskCompletion(1i32);
    pub const Failed: Print3DTaskCompletion = Print3DTaskCompletion(2i32);
    pub const Slicing: Print3DTaskCompletion = Print3DTaskCompletion(3i32);
    pub const Submitted: Print3DTaskCompletion = Print3DTaskCompletion(4i32);
}
impl ::std::convert::From<i32> for Print3DTaskCompletion {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Print3DTaskCompletion {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Print3DTaskCompletion {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Print3DTaskCompletion;i4)");
}
impl ::windows::runtime::DefaultType for Print3DTaskCompletion {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct Print3DTaskDetail(pub i32);
impl Print3DTaskDetail {
    pub const Unknown: Print3DTaskDetail = Print3DTaskDetail(0i32);
    pub const ModelExceedsPrintBed: Print3DTaskDetail = Print3DTaskDetail(1i32);
    pub const UploadFailed: Print3DTaskDetail = Print3DTaskDetail(2i32);
    pub const InvalidMaterialSelection: Print3DTaskDetail = Print3DTaskDetail(3i32);
    pub const InvalidModel: Print3DTaskDetail = Print3DTaskDetail(4i32);
    pub const ModelNotManifold: Print3DTaskDetail = Print3DTaskDetail(5i32);
    pub const InvalidPrintTicket: Print3DTaskDetail = Print3DTaskDetail(6i32);
}
impl ::std::convert::From<i32> for Print3DTaskDetail {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Print3DTaskDetail {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Print3DTaskDetail {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Print3DTaskDetail;i4)");
}
impl ::windows::runtime::DefaultType for Print3DTaskDetail {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Print3DTaskRequest(::windows::runtime::IInspectable);
impl Print3DTaskRequest {
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn CreateTask<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, Print3DTaskSourceRequestedHandler>>(&self, title: Param0, printerid: Param1, handler: Param2) -> ::windows::runtime::Result<Print3DTask> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), title.into_param().abi(), printerid.into_param().abi(), handler.into_param().abi(), &mut result__).from_abi::<Print3DTask>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Print3DTaskRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DTaskRequest;{2595c46f-2245-4c5a-8731-0d604dc6bc3c})");
}
unsafe impl ::windows::runtime::Interface for Print3DTaskRequest {
    type Vtable = IPrint3DTaskRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(630572143, 8773, 19546, [135, 49, 13, 96, 77, 198, 188, 60]);
}
impl ::windows::runtime::RuntimeName for Print3DTaskRequest {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskRequest";
}
impl ::std::convert::From<Print3DTaskRequest> for ::windows::runtime::IUnknown {
    fn from(value: Print3DTaskRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Print3DTaskRequest> for ::windows::runtime::IUnknown {
    fn from(value: &Print3DTaskRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Print3DTaskRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Print3DTaskRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Print3DTaskRequest> for ::windows::runtime::IInspectable {
    fn from(value: Print3DTaskRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Print3DTaskRequest> for ::windows::runtime::IInspectable {
    fn from(value: &Print3DTaskRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Print3DTaskRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Print3DTaskRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Print3DTaskRequest {}
unsafe impl ::std::marker::Sync for Print3DTaskRequest {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Print3DTaskRequestedEventArgs(::windows::runtime::IInspectable);
impl Print3DTaskRequestedEventArgs {
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<Print3DTaskRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Print3DTaskRequest>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Print3DTaskRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DTaskRequestedEventArgs;{150cb77f-18c5-40d7-9f40-fab3096e05a9})");
}
unsafe impl ::windows::runtime::Interface for Print3DTaskRequestedEventArgs {
    type Vtable = IPrint3DTaskRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(353154943, 6341, 16599, [159, 64, 250, 179, 9, 110, 5, 169]);
}
impl ::windows::runtime::RuntimeName for Print3DTaskRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskRequestedEventArgs";
}
impl ::std::convert::From<Print3DTaskRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: Print3DTaskRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Print3DTaskRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &Print3DTaskRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Print3DTaskRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Print3DTaskRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Print3DTaskRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: Print3DTaskRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Print3DTaskRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &Print3DTaskRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Print3DTaskRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Print3DTaskRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Print3DTaskRequestedEventArgs {}
unsafe impl ::std::marker::Sync for Print3DTaskRequestedEventArgs {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Print3DTaskSourceChangedEventArgs(::windows::runtime::IInspectable);
impl Print3DTaskSourceChangedEventArgs {
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Source(&self) -> ::windows::runtime::Result<Printing3D3MFPackage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3D3MFPackage>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Print3DTaskSourceChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DTaskSourceChangedEventArgs;{5bcd34af-24e9-4c10-8d07-14c346ba3fcf})");
}
unsafe impl ::windows::runtime::Interface for Print3DTaskSourceChangedEventArgs {
    type Vtable = IPrint3DTaskSourceChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1540175023, 9449, 19472, [141, 7, 20, 195, 70, 186, 63, 207]);
}
impl ::windows::runtime::RuntimeName for Print3DTaskSourceChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskSourceChangedEventArgs";
}
impl ::std::convert::From<Print3DTaskSourceChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: Print3DTaskSourceChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Print3DTaskSourceChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &Print3DTaskSourceChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Print3DTaskSourceChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Print3DTaskSourceChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Print3DTaskSourceChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: Print3DTaskSourceChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Print3DTaskSourceChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &Print3DTaskSourceChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Print3DTaskSourceChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Print3DTaskSourceChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Print3DTaskSourceChangedEventArgs {}
unsafe impl ::std::marker::Sync for Print3DTaskSourceChangedEventArgs {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Print3DTaskSourceRequestedArgs(::windows::runtime::IInspectable);
impl Print3DTaskSourceRequestedArgs {
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetSource<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3D3MFPackage>>(&self, source: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), source.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Print3DTaskSourceRequestedArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DTaskSourceRequestedArgs;{c77c9aba-24af-424d-a3bf-92250c355602})");
}
unsafe impl ::windows::runtime::Interface for Print3DTaskSourceRequestedArgs {
    type Vtable = IPrint3DTaskSourceRequestedArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3346832058, 9391, 16973, [163, 191, 146, 37, 12, 53, 86, 2]);
}
impl ::windows::runtime::RuntimeName for Print3DTaskSourceRequestedArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskSourceRequestedArgs";
}
impl ::std::convert::From<Print3DTaskSourceRequestedArgs> for ::windows::runtime::IUnknown {
    fn from(value: Print3DTaskSourceRequestedArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Print3DTaskSourceRequestedArgs> for ::windows::runtime::IUnknown {
    fn from(value: &Print3DTaskSourceRequestedArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Print3DTaskSourceRequestedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Print3DTaskSourceRequestedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Print3DTaskSourceRequestedArgs> for ::windows::runtime::IInspectable {
    fn from(value: Print3DTaskSourceRequestedArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Print3DTaskSourceRequestedArgs> for ::windows::runtime::IInspectable {
    fn from(value: &Print3DTaskSourceRequestedArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Print3DTaskSourceRequestedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Print3DTaskSourceRequestedArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Print3DTaskSourceRequestedArgs {}
unsafe impl ::std::marker::Sync for Print3DTaskSourceRequestedArgs {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Print3DTaskSourceRequestedHandler(::windows::runtime::IUnknown);
impl Print3DTaskSourceRequestedHandler {
    pub fn new<F: FnMut(&::std::option::Option<Print3DTaskSourceRequestedArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = Print3DTaskSourceRequestedHandler_box::<F> {
            vtable: &Print3DTaskSourceRequestedHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, Print3DTaskSourceRequestedArgs>>(&self, args: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), args.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Print3DTaskSourceRequestedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({e9175e70-c917-46de-bb51-d9a94db3711f})");
}
unsafe impl ::windows::runtime::Interface for Print3DTaskSourceRequestedHandler {
    type Vtable = Print3DTaskSourceRequestedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3910622832, 51479, 18142, [187, 81, 217, 169, 77, 179, 113, 31]);
}
#[repr(C)]
#[doc(hidden)]
pub struct Print3DTaskSourceRequestedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct Print3DTaskSourceRequestedHandler_box<F: FnMut(&::std::option::Option<Print3DTaskSourceRequestedArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const Print3DTaskSourceRequestedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<Print3DTaskSourceRequestedArgs>) -> ::windows::runtime::Result<()> + 'static> Print3DTaskSourceRequestedHandler_box<F> {
    const VTABLE: Print3DTaskSourceRequestedHandler_abi = Print3DTaskSourceRequestedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<Print3DTaskSourceRequestedHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, args: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&args as *const <Print3DTaskSourceRequestedArgs as ::windows::runtime::Abi>::Abi as *const <Print3DTaskSourceRequestedArgs as ::windows::runtime::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3D3MFPackage(::windows::runtime::IInspectable);
impl Printing3D3MFPackage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3D3MFPackage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`, `Storage_Streams`*"]
    pub fn SaveAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Storage_Streams`*"]
    pub fn PrintTicket(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Storage_Streams`*"]
    pub fn SetPrintTicket<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Storage_Streams`*"]
    pub fn ModelPart(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Storage_Streams`*"]
    pub fn SetModelPart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<Printing3DTextureResource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DTextureResource>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DTextureResource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn Textures(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DTextureResource>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DTextureResource>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`, `Storage_Streams`*"]
    pub fn LoadModelFromPackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Printing3DModel>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Printing3DModel>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn SaveModelToPackageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DModel>>(&self, value: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`, `Storage_Streams`*"]
    pub fn LoadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(value: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Printing3D3MFPackage>> {
        Self::IPrinting3D3MFPackageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Printing3D3MFPackage>>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Compression(&self) -> ::windows::runtime::Result<Printing3DPackageCompression> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3D3MFPackage2>(self)?;
        unsafe {
            let mut result__: Printing3DPackageCompression = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DPackageCompression>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetCompression(&self, value: Printing3DPackageCompression) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3D3MFPackage2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn IPrinting3D3MFPackageStatics<R, F: FnOnce(&IPrinting3D3MFPackageStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3D3MFPackage, IPrinting3D3MFPackageStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3D3MFPackage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3D3MFPackage;{f64dd5c8-2ab7-45a9-a1b7-267e948d5b18})");
}
unsafe impl ::windows::runtime::Interface for Printing3D3MFPackage {
    type Vtable = IPrinting3D3MFPackage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4132296136, 10935, 17833, [161, 183, 38, 126, 148, 141, 91, 24]);
}
impl ::windows::runtime::RuntimeName for Printing3D3MFPackage {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3D3MFPackage";
}
impl ::std::convert::From<Printing3D3MFPackage> for ::windows::runtime::IUnknown {
    fn from(value: Printing3D3MFPackage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3D3MFPackage> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3D3MFPackage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3D3MFPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3D3MFPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3D3MFPackage> for ::windows::runtime::IInspectable {
    fn from(value: Printing3D3MFPackage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3D3MFPackage> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3D3MFPackage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3D3MFPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3D3MFPackage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3D3MFPackage {}
unsafe impl ::std::marker::Sync for Printing3D3MFPackage {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DBaseMaterial(::windows::runtime::IInspectable);
impl Printing3DBaseMaterial {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DBaseMaterial, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Color(&self) -> ::windows::runtime::Result<Printing3DColorMaterial> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DColorMaterial>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetColor<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DColorMaterial>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Abs() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IPrinting3DBaseMaterialStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Pla() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IPrinting3DBaseMaterialStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IPrinting3DBaseMaterialStatics<R, F: FnOnce(&IPrinting3DBaseMaterialStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DBaseMaterial, IPrinting3DBaseMaterialStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DBaseMaterial {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DBaseMaterial;{d0f0e743-c50c-4bcb-9d04-fc16adcea2c9})");
}
unsafe impl ::windows::runtime::Interface for Printing3DBaseMaterial {
    type Vtable = IPrinting3DBaseMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3505448771, 50444, 19403, [157, 4, 252, 22, 173, 206, 162, 201]);
}
impl ::windows::runtime::RuntimeName for Printing3DBaseMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DBaseMaterial";
}
impl ::std::convert::From<Printing3DBaseMaterial> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DBaseMaterial) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DBaseMaterial> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DBaseMaterial) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DBaseMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DBaseMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DBaseMaterial> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DBaseMaterial) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DBaseMaterial> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DBaseMaterial) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DBaseMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DBaseMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DBaseMaterial {}
unsafe impl ::std::marker::Sync for Printing3DBaseMaterial {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DBaseMaterialGroup(::windows::runtime::IInspectable);
impl Printing3DBaseMaterialGroup {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn Bases(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DBaseMaterial>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DBaseMaterial>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn MaterialGroupId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Create(materialgroupid: u32) -> ::windows::runtime::Result<Printing3DBaseMaterialGroup> {
        Self::IPrinting3DBaseMaterialGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), materialgroupid, &mut result__).from_abi::<Printing3DBaseMaterialGroup>(result__)
        })
    }
    pub fn IPrinting3DBaseMaterialGroupFactory<R, F: FnOnce(&IPrinting3DBaseMaterialGroupFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DBaseMaterialGroup, IPrinting3DBaseMaterialGroupFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DBaseMaterialGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup;{94f070b8-2515-4a8d-a1f0-d0fc13d06021})");
}
unsafe impl ::windows::runtime::Interface for Printing3DBaseMaterialGroup {
    type Vtable = IPrinting3DBaseMaterialGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2498785464, 9493, 19085, [161, 240, 208, 252, 19, 208, 96, 33]);
}
impl ::windows::runtime::RuntimeName for Printing3DBaseMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup";
}
impl ::std::convert::From<Printing3DBaseMaterialGroup> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DBaseMaterialGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DBaseMaterialGroup> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DBaseMaterialGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DBaseMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DBaseMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DBaseMaterialGroup> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DBaseMaterialGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DBaseMaterialGroup> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DBaseMaterialGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DBaseMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DBaseMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DBaseMaterialGroup {}
unsafe impl ::std::marker::Sync for Printing3DBaseMaterialGroup {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Graphics_Printing3D`*"]
pub struct Printing3DBufferDescription {
    pub Format: Printing3DBufferFormat,
    pub Stride: u32,
}
impl Printing3DBufferDescription {}
impl ::std::default::Default for Printing3DBufferDescription {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Printing3DBufferDescription {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Printing3DBufferDescription").field("Format", &self.Format).field("Stride", &self.Stride).finish()
    }
}
impl ::std::cmp::PartialEq for Printing3DBufferDescription {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.Stride == other.Stride
    }
}
impl ::std::cmp::Eq for Printing3DBufferDescription {}
unsafe impl ::windows::runtime::Abi for Printing3DBufferDescription {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DBufferDescription {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Graphics.Printing3D.Printing3DBufferDescription;enum(Windows.Graphics.Printing3D.Printing3DBufferFormat;i4);u4)");
}
impl ::windows::runtime::DefaultType for Printing3DBufferDescription {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct Printing3DBufferFormat(pub i32);
impl Printing3DBufferFormat {
    pub const Unknown: Printing3DBufferFormat = Printing3DBufferFormat(0i32);
    pub const R32G32B32A32Float: Printing3DBufferFormat = Printing3DBufferFormat(2i32);
    pub const R32G32B32A32UInt: Printing3DBufferFormat = Printing3DBufferFormat(3i32);
    pub const R32G32B32Float: Printing3DBufferFormat = Printing3DBufferFormat(6i32);
    pub const R32G32B32UInt: Printing3DBufferFormat = Printing3DBufferFormat(7i32);
    pub const Printing3DDouble: Printing3DBufferFormat = Printing3DBufferFormat(500i32);
    pub const Printing3DUInt: Printing3DBufferFormat = Printing3DBufferFormat(501i32);
}
impl ::std::convert::From<i32> for Printing3DBufferFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Printing3DBufferFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DBufferFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DBufferFormat;i4)");
}
impl ::windows::runtime::DefaultType for Printing3DBufferFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DColorMaterial(::windows::runtime::IInspectable);
impl Printing3DColorMaterial {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DColorMaterial, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetValue(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Graphics_Printing3D`, `UI`*"]
    pub fn Color(&self) -> ::windows::runtime::Result<super::super::UI::Color> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3DColorMaterial2>(self)?;
        unsafe {
            let mut result__: super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        }
    }
    #[cfg(feature = "UI")]
    #[doc = "*Required features: `Graphics_Printing3D`, `UI`*"]
    pub fn SetColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::UI::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3DColorMaterial2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DColorMaterial {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DColorMaterial;{e1899928-7ce7-4285-a35d-f145c9510c7b})");
}
unsafe impl ::windows::runtime::Interface for Printing3DColorMaterial {
    type Vtable = IPrinting3DColorMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3783891240, 31975, 17029, [163, 93, 241, 69, 201, 81, 12, 123]);
}
impl ::windows::runtime::RuntimeName for Printing3DColorMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DColorMaterial";
}
impl ::std::convert::From<Printing3DColorMaterial> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DColorMaterial) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DColorMaterial> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DColorMaterial) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DColorMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DColorMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DColorMaterial> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DColorMaterial) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DColorMaterial> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DColorMaterial) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DColorMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DColorMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DColorMaterial {}
unsafe impl ::std::marker::Sync for Printing3DColorMaterial {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DColorMaterialGroup(::windows::runtime::IInspectable);
impl Printing3DColorMaterialGroup {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn Colors(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DColorMaterial>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DColorMaterial>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn MaterialGroupId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Create(materialgroupid: u32) -> ::windows::runtime::Result<Printing3DColorMaterialGroup> {
        Self::IPrinting3DColorMaterialGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), materialgroupid, &mut result__).from_abi::<Printing3DColorMaterialGroup>(result__)
        })
    }
    pub fn IPrinting3DColorMaterialGroupFactory<R, F: FnOnce(&IPrinting3DColorMaterialGroupFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DColorMaterialGroup, IPrinting3DColorMaterialGroupFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DColorMaterialGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DColorMaterialGroup;{001a6bd0-aadf-4226-afe9-f369a0b45004})");
}
unsafe impl ::windows::runtime::Interface for Printing3DColorMaterialGroup {
    type Vtable = IPrinting3DColorMaterialGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1731536, 43743, 16934, [175, 233, 243, 105, 160, 180, 80, 4]);
}
impl ::windows::runtime::RuntimeName for Printing3DColorMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DColorMaterialGroup";
}
impl ::std::convert::From<Printing3DColorMaterialGroup> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DColorMaterialGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DColorMaterialGroup> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DColorMaterialGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DColorMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DColorMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DColorMaterialGroup> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DColorMaterialGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DColorMaterialGroup> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DColorMaterialGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DColorMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DColorMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DColorMaterialGroup {}
unsafe impl ::std::marker::Sync for Printing3DColorMaterialGroup {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DComponent(::windows::runtime::IInspectable);
impl Printing3DComponent {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DComponent, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Mesh(&self) -> ::windows::runtime::Result<Printing3DMesh> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DMesh>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetMesh<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DMesh>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn Components(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DComponentWithMatrix>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DComponentWithMatrix>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Thumbnail(&self) -> ::windows::runtime::Result<Printing3DTextureResource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DTextureResource>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DTextureResource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<Printing3DObjectType> {
        let this = self;
        unsafe {
            let mut result__: Printing3DObjectType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DObjectType>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetType(&self, value: Printing3DObjectType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn PartNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetPartNumber<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DComponent {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DComponent;{7e287845-bf7f-4cdb-a27f-30a01437fede})");
}
unsafe impl ::windows::runtime::Interface for Printing3DComponent {
    type Vtable = IPrinting3DComponent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2116581445, 49023, 19675, [162, 127, 48, 160, 20, 55, 254, 222]);
}
impl ::windows::runtime::RuntimeName for Printing3DComponent {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DComponent";
}
impl ::std::convert::From<Printing3DComponent> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DComponent) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DComponent> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DComponent) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DComponent> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DComponent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DComponent> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DComponent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DComponent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DComponent {}
unsafe impl ::std::marker::Sync for Printing3DComponent {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DComponentWithMatrix(::windows::runtime::IInspectable);
impl Printing3DComponentWithMatrix {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DComponentWithMatrix, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Component(&self) -> ::windows::runtime::Result<Printing3DComponent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DComponent>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetComponent<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DComponent>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Numerics`*"]
    pub fn Matrix(&self) -> ::windows::runtime::Result<super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Numerics::Matrix4x4 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Numerics::Matrix4x4>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Numerics`*"]
    pub fn SetMatrix<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Numerics::Matrix4x4>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DComponentWithMatrix {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DComponentWithMatrix;{3279f335-0ef0-456b-9a21-49bebe8b51c2})");
}
unsafe impl ::windows::runtime::Interface for Printing3DComponentWithMatrix {
    type Vtable = IPrinting3DComponentWithMatrix_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(846852917, 3824, 17771, [154, 33, 73, 190, 190, 139, 81, 194]);
}
impl ::windows::runtime::RuntimeName for Printing3DComponentWithMatrix {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DComponentWithMatrix";
}
impl ::std::convert::From<Printing3DComponentWithMatrix> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DComponentWithMatrix) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DComponentWithMatrix> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DComponentWithMatrix) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DComponentWithMatrix {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DComponentWithMatrix {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DComponentWithMatrix> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DComponentWithMatrix) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DComponentWithMatrix> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DComponentWithMatrix) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DComponentWithMatrix {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DComponentWithMatrix {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DComponentWithMatrix {}
unsafe impl ::std::marker::Sync for Printing3DComponentWithMatrix {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DCompositeMaterial(::windows::runtime::IInspectable);
impl Printing3DCompositeMaterial {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DCompositeMaterial, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn Values(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<f64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<f64>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DCompositeMaterial {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DCompositeMaterial;{462238dd-562e-4f6c-882d-f4d841fd63c7})");
}
unsafe impl ::windows::runtime::Interface for Printing3DCompositeMaterial {
    type Vtable = IPrinting3DCompositeMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1176647901, 22062, 20332, [136, 45, 244, 216, 65, 253, 99, 199]);
}
impl ::windows::runtime::RuntimeName for Printing3DCompositeMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DCompositeMaterial";
}
impl ::std::convert::From<Printing3DCompositeMaterial> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DCompositeMaterial) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DCompositeMaterial> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DCompositeMaterial) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DCompositeMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DCompositeMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DCompositeMaterial> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DCompositeMaterial) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DCompositeMaterial> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DCompositeMaterial) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DCompositeMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DCompositeMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DCompositeMaterial {}
unsafe impl ::std::marker::Sync for Printing3DCompositeMaterial {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DCompositeMaterialGroup(::windows::runtime::IInspectable);
impl Printing3DCompositeMaterialGroup {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn Composites(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterial>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterial>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn MaterialGroupId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn MaterialIndices(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Create(materialgroupid: u32) -> ::windows::runtime::Result<Printing3DCompositeMaterialGroup> {
        Self::IPrinting3DCompositeMaterialGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), materialgroupid, &mut result__).from_abi::<Printing3DCompositeMaterialGroup>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn BaseMaterialGroup(&self) -> ::windows::runtime::Result<Printing3DBaseMaterialGroup> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3DCompositeMaterialGroup2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DBaseMaterialGroup>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetBaseMaterialGroup<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DBaseMaterialGroup>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3DCompositeMaterialGroup2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IPrinting3DCompositeMaterialGroupFactory<R, F: FnOnce(&IPrinting3DCompositeMaterialGroupFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DCompositeMaterialGroup, IPrinting3DCompositeMaterialGroupFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DCompositeMaterialGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup;{8d946a5b-40f1-496d-a5fb-340a5a678e30})");
}
unsafe impl ::windows::runtime::Interface for Printing3DCompositeMaterialGroup {
    type Vtable = IPrinting3DCompositeMaterialGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2375314011, 16625, 18797, [165, 251, 52, 10, 90, 103, 142, 48]);
}
impl ::windows::runtime::RuntimeName for Printing3DCompositeMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup";
}
impl ::std::convert::From<Printing3DCompositeMaterialGroup> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DCompositeMaterialGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DCompositeMaterialGroup> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DCompositeMaterialGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DCompositeMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DCompositeMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DCompositeMaterialGroup> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DCompositeMaterialGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DCompositeMaterialGroup> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DCompositeMaterialGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DCompositeMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DCompositeMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DCompositeMaterialGroup {}
unsafe impl ::std::marker::Sync for Printing3DCompositeMaterialGroup {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct Printing3DContract(pub u8);
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DFaceReductionOptions(::windows::runtime::IInspectable);
impl Printing3DFaceReductionOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DFaceReductionOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn MaxReductionArea(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetMaxReductionArea(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn TargetTriangleCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetTargetTriangleCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn MaxEdgeLength(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetMaxEdgeLength(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DFaceReductionOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DFaceReductionOptions;{bbfed397-2d74-46f7-be85-99a67bbb6629})");
}
unsafe impl ::windows::runtime::Interface for Printing3DFaceReductionOptions {
    type Vtable = IPrinting3DFaceReductionOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3154039703, 11636, 18167, [190, 133, 153, 166, 123, 187, 102, 41]);
}
impl ::windows::runtime::RuntimeName for Printing3DFaceReductionOptions {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DFaceReductionOptions";
}
impl ::std::convert::From<Printing3DFaceReductionOptions> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DFaceReductionOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DFaceReductionOptions> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DFaceReductionOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DFaceReductionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DFaceReductionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DFaceReductionOptions> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DFaceReductionOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DFaceReductionOptions> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DFaceReductionOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DFaceReductionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DFaceReductionOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DFaceReductionOptions {}
unsafe impl ::std::marker::Sync for Printing3DFaceReductionOptions {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DMaterial(::windows::runtime::IInspectable);
impl Printing3DMaterial {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DMaterial, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn BaseGroups(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DBaseMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DBaseMaterialGroup>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn ColorGroups(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DColorMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DColorMaterialGroup>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn Texture2CoordGroups(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterialGroup>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn CompositeGroups(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterialGroup>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn MultiplePropertyGroups(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterialGroup>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DMaterial {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DMaterial;{378db256-ed62-4952-b85b-03567d7c465e})");
}
unsafe impl ::windows::runtime::Interface for Printing3DMaterial {
    type Vtable = IPrinting3DMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(932033110, 60770, 18770, [184, 91, 3, 86, 125, 124, 70, 94]);
}
impl ::windows::runtime::RuntimeName for Printing3DMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMaterial";
}
impl ::std::convert::From<Printing3DMaterial> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DMaterial) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DMaterial> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DMaterial) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DMaterial> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DMaterial) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DMaterial> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DMaterial) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DMaterial {}
unsafe impl ::std::marker::Sync for Printing3DMaterial {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DMesh(::windows::runtime::IInspectable);
impl Printing3DMesh {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DMesh, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn VertexCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetVertexCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn IndexCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetIndexCount(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn VertexPositionsDescription(&self) -> ::windows::runtime::Result<Printing3DBufferDescription> {
        let this = self;
        unsafe {
            let mut result__: Printing3DBufferDescription = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DBufferDescription>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetVertexPositionsDescription<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DBufferDescription>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn VertexNormalsDescription(&self) -> ::windows::runtime::Result<Printing3DBufferDescription> {
        let this = self;
        unsafe {
            let mut result__: Printing3DBufferDescription = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DBufferDescription>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetVertexNormalsDescription<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DBufferDescription>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn TriangleIndicesDescription(&self) -> ::windows::runtime::Result<Printing3DBufferDescription> {
        let this = self;
        unsafe {
            let mut result__: Printing3DBufferDescription = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DBufferDescription>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetTriangleIndicesDescription<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DBufferDescription>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn TriangleMaterialIndicesDescription(&self) -> ::windows::runtime::Result<Printing3DBufferDescription> {
        let this = self;
        unsafe {
            let mut result__: Printing3DBufferDescription = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DBufferDescription>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetTriangleMaterialIndicesDescription<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DBufferDescription>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Storage_Streams`*"]
    pub fn GetVertexPositions(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn CreateVertexPositions(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Storage_Streams`*"]
    pub fn GetVertexNormals(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn CreateVertexNormals(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Storage_Streams`*"]
    pub fn GetTriangleIndices(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn CreateTriangleIndices(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Storage_Streams`*"]
    pub fn GetTriangleMaterialIndices(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn CreateTriangleMaterialIndices(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn BufferDescriptionSet(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn BufferSet(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn VerifyAsync(&self, value: Printing3DMeshVerificationMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Printing3DMeshVerificationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<Printing3DMeshVerificationResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DMesh {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DMesh;{192e90dc-0228-2e01-bc20-c5290cbf32c4})");
}
unsafe impl ::windows::runtime::Interface for Printing3DMesh {
    type Vtable = IPrinting3DMesh_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(422482140, 552, 11777, [188, 32, 197, 41, 12, 191, 50, 196]);
}
impl ::windows::runtime::RuntimeName for Printing3DMesh {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMesh";
}
impl ::std::convert::From<Printing3DMesh> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DMesh) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DMesh> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DMesh) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DMesh> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DMesh) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DMesh> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DMesh) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DMesh {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DMesh {}
unsafe impl ::std::marker::Sync for Printing3DMesh {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct Printing3DMeshVerificationMode(pub i32);
impl Printing3DMeshVerificationMode {
    pub const FindFirstError: Printing3DMeshVerificationMode = Printing3DMeshVerificationMode(0i32);
    pub const FindAllErrors: Printing3DMeshVerificationMode = Printing3DMeshVerificationMode(1i32);
}
impl ::std::convert::From<i32> for Printing3DMeshVerificationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Printing3DMeshVerificationMode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DMeshVerificationMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DMeshVerificationMode;i4)");
}
impl ::windows::runtime::DefaultType for Printing3DMeshVerificationMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DMeshVerificationResult(::windows::runtime::IInspectable);
impl Printing3DMeshVerificationResult {
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn IsValid(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn NonmanifoldTriangles(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn ReversedNormalTriangles(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DMeshVerificationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DMeshVerificationResult;{195671ba-e93a-4e8a-a46f-dea8e852197e})");
}
unsafe impl ::windows::runtime::Interface for Printing3DMeshVerificationResult {
    type Vtable = IPrinting3DMeshVerificationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(425095610, 59706, 20106, [164, 111, 222, 168, 232, 82, 25, 126]);
}
impl ::windows::runtime::RuntimeName for Printing3DMeshVerificationResult {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMeshVerificationResult";
}
impl ::std::convert::From<Printing3DMeshVerificationResult> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DMeshVerificationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DMeshVerificationResult> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DMeshVerificationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DMeshVerificationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DMeshVerificationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DMeshVerificationResult> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DMeshVerificationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DMeshVerificationResult> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DMeshVerificationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DMeshVerificationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DMeshVerificationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DMeshVerificationResult {}
unsafe impl ::std::marker::Sync for Printing3DMeshVerificationResult {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DModel(::windows::runtime::IInspectable);
impl Printing3DModel {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DModel, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Unit(&self) -> ::windows::runtime::Result<Printing3DModelUnit> {
        let this = self;
        unsafe {
            let mut result__: Printing3DModelUnit = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DModelUnit>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetUnit(&self, value: Printing3DModelUnit) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn Textures(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DModelTexture>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DModelTexture>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn Meshes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DMesh>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DMesh>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn Components(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DComponent>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DComponent>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Material(&self) -> ::windows::runtime::Result<Printing3DMaterial> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DMaterial>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetMaterial<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DMaterial>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Build(&self) -> ::windows::runtime::Result<Printing3DComponent> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DComponent>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetBuild<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DComponent>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Version(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetVersion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn RequiredExtensions(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn Metadata(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn RepairAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Clone(&self) -> ::windows::runtime::Result<Printing3DModel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DModel>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn TryPartialRepairAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn TryPartialRepairWithTimeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, maxwaittime: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), maxwaittime.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn TryReduceFacesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn TryReduceFacesWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DFaceReductionOptions>>(&self, printing3dfacereductionoptions: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), printing3dfacereductionoptions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn TryReduceFacesWithOptionsAndTimeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DFaceReductionOptions>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, printing3dfacereductionoptions: Param0, maxwait: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), printing3dfacereductionoptions.into_param().abi(), maxwait.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation`*"]
    pub fn RepairWithProgressAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DModel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DModel;{2d012ef0-52fb-919a-77b0-4b1a3b80324f})");
}
unsafe impl ::windows::runtime::Interface for Printing3DModel {
    type Vtable = IPrinting3DModel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(755052272, 21243, 37274, [119, 176, 75, 26, 59, 128, 50, 79]);
}
impl ::windows::runtime::RuntimeName for Printing3DModel {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DModel";
}
impl ::std::convert::From<Printing3DModel> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DModel) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DModel> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DModel) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DModel> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DModel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DModel> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DModel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DModel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DModel {}
unsafe impl ::std::marker::Sync for Printing3DModel {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DModelTexture(::windows::runtime::IInspectable);
impl Printing3DModelTexture {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DModelTexture, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn TextureResource(&self) -> ::windows::runtime::Result<Printing3DTextureResource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DTextureResource>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetTextureResource<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DTextureResource>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn TileStyleU(&self) -> ::windows::runtime::Result<Printing3DTextureEdgeBehavior> {
        let this = self;
        unsafe {
            let mut result__: Printing3DTextureEdgeBehavior = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DTextureEdgeBehavior>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetTileStyleU(&self, value: Printing3DTextureEdgeBehavior) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn TileStyleV(&self) -> ::windows::runtime::Result<Printing3DTextureEdgeBehavior> {
        let this = self;
        unsafe {
            let mut result__: Printing3DTextureEdgeBehavior = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DTextureEdgeBehavior>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetTileStyleV(&self, value: Printing3DTextureEdgeBehavior) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DModelTexture {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DModelTexture;{5dafcf01-b59d-483c-97bb-a4d546d1c75c})");
}
unsafe impl ::windows::runtime::Interface for Printing3DModelTexture {
    type Vtable = IPrinting3DModelTexture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1571802881, 46493, 18492, [151, 187, 164, 213, 70, 209, 199, 92]);
}
impl ::windows::runtime::RuntimeName for Printing3DModelTexture {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DModelTexture";
}
impl ::std::convert::From<Printing3DModelTexture> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DModelTexture) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DModelTexture> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DModelTexture) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DModelTexture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DModelTexture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DModelTexture> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DModelTexture) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DModelTexture> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DModelTexture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DModelTexture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DModelTexture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DModelTexture {}
unsafe impl ::std::marker::Sync for Printing3DModelTexture {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct Printing3DModelUnit(pub i32);
impl Printing3DModelUnit {
    pub const Meter: Printing3DModelUnit = Printing3DModelUnit(0i32);
    pub const Micron: Printing3DModelUnit = Printing3DModelUnit(1i32);
    pub const Millimeter: Printing3DModelUnit = Printing3DModelUnit(2i32);
    pub const Centimeter: Printing3DModelUnit = Printing3DModelUnit(3i32);
    pub const Inch: Printing3DModelUnit = Printing3DModelUnit(4i32);
    pub const Foot: Printing3DModelUnit = Printing3DModelUnit(5i32);
}
impl ::std::convert::From<i32> for Printing3DModelUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Printing3DModelUnit {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DModelUnit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DModelUnit;i4)");
}
impl ::windows::runtime::DefaultType for Printing3DModelUnit {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DMultiplePropertyMaterial(::windows::runtime::IInspectable);
impl Printing3DMultiplePropertyMaterial {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DMultiplePropertyMaterial, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn MaterialIndices(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DMultiplePropertyMaterial {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial;{25a6254b-c6e9-484d-a214-a25e5776ba62})");
}
unsafe impl ::windows::runtime::Interface for Printing3DMultiplePropertyMaterial {
    type Vtable = IPrinting3DMultiplePropertyMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(631645515, 50921, 18509, [162, 20, 162, 94, 87, 118, 186, 98]);
}
impl ::windows::runtime::RuntimeName for Printing3DMultiplePropertyMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial";
}
impl ::std::convert::From<Printing3DMultiplePropertyMaterial> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DMultiplePropertyMaterial) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DMultiplePropertyMaterial> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DMultiplePropertyMaterial) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DMultiplePropertyMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DMultiplePropertyMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DMultiplePropertyMaterial> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DMultiplePropertyMaterial) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DMultiplePropertyMaterial> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DMultiplePropertyMaterial) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DMultiplePropertyMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DMultiplePropertyMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DMultiplePropertyMaterial {}
unsafe impl ::std::marker::Sync for Printing3DMultiplePropertyMaterial {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DMultiplePropertyMaterialGroup(::windows::runtime::IInspectable);
impl Printing3DMultiplePropertyMaterialGroup {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn MultipleProperties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterial>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterial>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn MaterialGroupIndices(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn MaterialGroupId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Create(materialgroupid: u32) -> ::windows::runtime::Result<Printing3DMultiplePropertyMaterialGroup> {
        Self::IPrinting3DMultiplePropertyMaterialGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), materialgroupid, &mut result__).from_abi::<Printing3DMultiplePropertyMaterialGroup>(result__)
        })
    }
    pub fn IPrinting3DMultiplePropertyMaterialGroupFactory<R, F: FnOnce(&IPrinting3DMultiplePropertyMaterialGroupFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DMultiplePropertyMaterialGroup, IPrinting3DMultiplePropertyMaterialGroupFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DMultiplePropertyMaterialGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup;{f0950519-aeb9-4515-a39b-a088fbbb277c})");
}
unsafe impl ::windows::runtime::Interface for Printing3DMultiplePropertyMaterialGroup {
    type Vtable = IPrinting3DMultiplePropertyMaterialGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4036298009, 44729, 17685, [163, 155, 160, 136, 251, 187, 39, 124]);
}
impl ::windows::runtime::RuntimeName for Printing3DMultiplePropertyMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup";
}
impl ::std::convert::From<Printing3DMultiplePropertyMaterialGroup> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DMultiplePropertyMaterialGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DMultiplePropertyMaterialGroup> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DMultiplePropertyMaterialGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DMultiplePropertyMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DMultiplePropertyMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DMultiplePropertyMaterialGroup> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DMultiplePropertyMaterialGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DMultiplePropertyMaterialGroup> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DMultiplePropertyMaterialGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DMultiplePropertyMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DMultiplePropertyMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DMultiplePropertyMaterialGroup {}
unsafe impl ::std::marker::Sync for Printing3DMultiplePropertyMaterialGroup {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct Printing3DObjectType(pub i32);
impl Printing3DObjectType {
    pub const Model: Printing3DObjectType = Printing3DObjectType(0i32);
    pub const Support: Printing3DObjectType = Printing3DObjectType(1i32);
    pub const Others: Printing3DObjectType = Printing3DObjectType(2i32);
}
impl ::std::convert::From<i32> for Printing3DObjectType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Printing3DObjectType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DObjectType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DObjectType;i4)");
}
impl ::windows::runtime::DefaultType for Printing3DObjectType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct Printing3DPackageCompression(pub i32);
impl Printing3DPackageCompression {
    pub const Low: Printing3DPackageCompression = Printing3DPackageCompression(0i32);
    pub const Medium: Printing3DPackageCompression = Printing3DPackageCompression(1i32);
    pub const High: Printing3DPackageCompression = Printing3DPackageCompression(2i32);
}
impl ::std::convert::From<i32> for Printing3DPackageCompression {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Printing3DPackageCompression {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DPackageCompression {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DPackageCompression;i4)");
}
impl ::windows::runtime::DefaultType for Printing3DPackageCompression {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DTexture2CoordMaterial(::windows::runtime::IInspectable);
impl Printing3DTexture2CoordMaterial {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DTexture2CoordMaterial, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Texture(&self) -> ::windows::runtime::Result<Printing3DModelTexture> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DModelTexture>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetTexture<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DModelTexture>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn U(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetU(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn V(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetV(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DTexture2CoordMaterial {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial;{8d844bfb-07e9-4986-9833-8dd3d48c6859})");
}
unsafe impl ::windows::runtime::Interface for Printing3DTexture2CoordMaterial {
    type Vtable = IPrinting3DTexture2CoordMaterial_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2374257659, 2025, 18822, [152, 51, 141, 211, 212, 140, 104, 89]);
}
impl ::windows::runtime::RuntimeName for Printing3DTexture2CoordMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial";
}
impl ::std::convert::From<Printing3DTexture2CoordMaterial> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DTexture2CoordMaterial) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DTexture2CoordMaterial> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DTexture2CoordMaterial) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DTexture2CoordMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DTexture2CoordMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DTexture2CoordMaterial> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DTexture2CoordMaterial) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DTexture2CoordMaterial> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DTexture2CoordMaterial) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DTexture2CoordMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DTexture2CoordMaterial {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DTexture2CoordMaterial {}
unsafe impl ::std::marker::Sync for Printing3DTexture2CoordMaterial {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DTexture2CoordMaterialGroup(::windows::runtime::IInspectable);
impl Printing3DTexture2CoordMaterialGroup {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Foundation_Collections`*"]
    pub fn Texture2Coords(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterial>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterial>>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn MaterialGroupId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Create(materialgroupid: u32) -> ::windows::runtime::Result<Printing3DTexture2CoordMaterialGroup> {
        Self::IPrinting3DTexture2CoordMaterialGroupFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), materialgroupid, &mut result__).from_abi::<Printing3DTexture2CoordMaterialGroup>(result__)
        })
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Texture(&self) -> ::windows::runtime::Result<Printing3DModelTexture> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3DTexture2CoordMaterialGroup2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Printing3DModelTexture>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetTexture<'a, Param0: ::windows::runtime::IntoParam<'a, Printing3DModelTexture>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPrinting3DTexture2CoordMaterialGroup2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IPrinting3DTexture2CoordMaterialGroupFactory<R, F: FnOnce(&IPrinting3DTexture2CoordMaterialGroupFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DTexture2CoordMaterialGroup, IPrinting3DTexture2CoordMaterialGroupFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DTexture2CoordMaterialGroup {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup;{627d7ca7-6d90-4fb9-9fc4-9feff3dfa892})");
}
unsafe impl ::windows::runtime::Interface for Printing3DTexture2CoordMaterialGroup {
    type Vtable = IPrinting3DTexture2CoordMaterialGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1652391079, 28048, 20409, [159, 196, 159, 239, 243, 223, 168, 146]);
}
impl ::windows::runtime::RuntimeName for Printing3DTexture2CoordMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup";
}
impl ::std::convert::From<Printing3DTexture2CoordMaterialGroup> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DTexture2CoordMaterialGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DTexture2CoordMaterialGroup> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DTexture2CoordMaterialGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DTexture2CoordMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DTexture2CoordMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DTexture2CoordMaterialGroup> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DTexture2CoordMaterialGroup) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DTexture2CoordMaterialGroup> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DTexture2CoordMaterialGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DTexture2CoordMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DTexture2CoordMaterialGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DTexture2CoordMaterialGroup {}
unsafe impl ::std::marker::Sync for Printing3DTexture2CoordMaterialGroup {}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct Printing3DTextureEdgeBehavior(pub i32);
impl Printing3DTextureEdgeBehavior {
    pub const None: Printing3DTextureEdgeBehavior = Printing3DTextureEdgeBehavior(0i32);
    pub const Wrap: Printing3DTextureEdgeBehavior = Printing3DTextureEdgeBehavior(1i32);
    pub const Mirror: Printing3DTextureEdgeBehavior = Printing3DTextureEdgeBehavior(2i32);
    pub const Clamp: Printing3DTextureEdgeBehavior = Printing3DTextureEdgeBehavior(3i32);
}
impl ::std::convert::From<i32> for Printing3DTextureEdgeBehavior {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Printing3DTextureEdgeBehavior {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DTextureEdgeBehavior {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DTextureEdgeBehavior;i4)");
}
impl ::windows::runtime::DefaultType for Printing3DTextureEdgeBehavior {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing3D`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct Printing3DTextureResource(::windows::runtime::IInspectable);
impl Printing3DTextureResource {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Printing3DTextureResource, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Storage_Streams`*"]
    pub fn TextureData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Graphics_Printing3D`, `Storage_Streams`*"]
    pub fn SetTextureData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing3D`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Printing3DTextureResource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DTextureResource;{a70df32d-6ab1-44ae-bc45-a27382c0d38c})");
}
unsafe impl ::windows::runtime::Interface for Printing3DTextureResource {
    type Vtable = IPrinting3DTextureResource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2802709293, 27313, 17582, [188, 69, 162, 115, 130, 192, 211, 140]);
}
impl ::windows::runtime::RuntimeName for Printing3DTextureResource {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DTextureResource";
}
impl ::std::convert::From<Printing3DTextureResource> for ::windows::runtime::IUnknown {
    fn from(value: Printing3DTextureResource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Printing3DTextureResource> for ::windows::runtime::IUnknown {
    fn from(value: &Printing3DTextureResource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Printing3DTextureResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Printing3DTextureResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<Printing3DTextureResource> for ::windows::runtime::IInspectable {
    fn from(value: Printing3DTextureResource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Printing3DTextureResource> for ::windows::runtime::IInspectable {
    fn from(value: &Printing3DTextureResource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Printing3DTextureResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Printing3DTextureResource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Printing3DTextureResource {}
unsafe impl ::std::marker::Sync for Printing3DTextureResource {}
