#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbBulkInEndpointDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbBulkInEndpointDescriptor {
    type Vtable = IUsbBulkInEndpointDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c6e4846_06cf_42a9_9dc2_971c1b14b6e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbBulkInEndpointDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbBulkInPipe(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbBulkInPipe {
    type Vtable = IUsbBulkInPipeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf01d2d3b_4548_4d50_b326_d82cdabe1220);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbBulkInPipeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UsbReadOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UsbReadOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbBulkOutEndpointDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbBulkOutEndpointDescriptor {
    type Vtable = IUsbBulkOutEndpointDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2820847a_ffee_4f60_9be1_956cac3ecb65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbBulkOutEndpointDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbBulkOutPipe(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbBulkOutPipe {
    type Vtable = IUsbBulkOutPipeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8e9ee6e_0115_45aa_8b21_37b225bccee7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbBulkOutPipeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UsbWriteOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UsbWriteOptions) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbConfiguration {
    type Vtable = IUsbConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68177429_36a9_46d7_b873_fc689251ec30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbConfigurationDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbConfigurationDescriptor {
    type Vtable = IUsbConfigurationDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2176d92_b442_407a_8207_7d646c0385f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbConfigurationDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbConfigurationDescriptorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbConfigurationDescriptorStatics {
    type Vtable = IUsbConfigurationDescriptorStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x424ced93_e740_40a1_92bd_da120ea04914);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbConfigurationDescriptorStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, parsed: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbControlRequestType(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbControlRequestType {
    type Vtable = IUsbControlRequestTypeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e9465a6_d73d_46de_94be_aae7f07c0f5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbControlRequestTypeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UsbTransferDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UsbTransferDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UsbControlTransferType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UsbControlTransferType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UsbControlRecipient) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UsbControlRecipient) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbDescriptor {
    type Vtable = IUsbDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a89f216_5f9d_4874_8904_da9ad3f5528f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbDevice {
    type Vtable = IUsbDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5249b992_c456_44d5_ad5e_24f5a089f63b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbDeviceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setuppacket: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setuppacket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setuppacket: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, setuppacket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbDeviceClass(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbDeviceClass {
    type Vtable = IUsbDeviceClassVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x051942f9_845e_47eb_b12a_38f2f617afe7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbDeviceClassVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbDeviceClasses(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbDeviceClasses {
    type Vtable = IUsbDeviceClassesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x686f955d_9b92_4b30_9781_c22c55ac35cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbDeviceClassesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbDeviceClassesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbDeviceClassesStatics {
    type Vtable = IUsbDeviceClassesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb20b0527_c580_4599_a165_981b4fd03230);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbDeviceClassesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbDeviceDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbDeviceDescriptor {
    type Vtable = IUsbDeviceDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f48d1f6_ba97_4322_b92c_b5b189216588);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbDeviceDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbDeviceStatics {
    type Vtable = IUsbDeviceStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x066b85a2_09b7_4446_8502_6fe6dcaa7309);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbDeviceStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendorid: u32, productid: u32, winusbinterfaceclass: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, winusbinterfaceclass: ::windows::core::GUID, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendorid: u32, productid: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usbclass: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbEndpointDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbEndpointDescriptor {
    type Vtable = IUsbEndpointDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b4862d9_8df7_4b40_ac83_578f139f0575);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbEndpointDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UsbTransferDirection) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UsbEndpointType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbEndpointDescriptorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbEndpointDescriptorStatics {
    type Vtable = IUsbEndpointDescriptorStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc890b201_9a6a_495e_a82c_295b9e708106);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbEndpointDescriptorStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, parsed: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbInterface(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbInterface {
    type Vtable = IUsbInterfaceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0322b95_7f47_48ab_a727_678c25be2112);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbInterfaceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbInterfaceDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbInterfaceDescriptor {
    type Vtable = IUsbInterfaceDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x199670c7_b7ee_4f90_8cd5_94a2e257598a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbInterfaceDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbInterfaceDescriptorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbInterfaceDescriptorStatics {
    type Vtable = IUsbInterfaceDescriptorStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe34a9ff5_77d6_48b6_b0be_16c6422316fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbInterfaceDescriptorStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, parsed: *mut ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbInterfaceSetting(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbInterfaceSetting {
    type Vtable = IUsbInterfaceSettingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1827bba7_8da7_4af7_8f4c_7f3032e781f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbInterfaceSettingVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbInterruptInEndpointDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbInterruptInEndpointDescriptor {
    type Vtable = IUsbInterruptInEndpointDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0528967_c911_4c3a_86b2_419c2da89039);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbInterruptInEndpointDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbInterruptInEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbInterruptInEventArgs {
    type Vtable = IUsbInterruptInEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7b04092_1418_4936_8209_299cf5605583);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbInterruptInEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbInterruptInPipe(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbInterruptInPipe {
    type Vtable = IUsbInterruptInPipeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa007116_84d7_48c7_8a3f_4c0b235f2ea6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbInterruptInPipeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbInterruptOutEndpointDescriptor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbInterruptOutEndpointDescriptor {
    type Vtable = IUsbInterruptOutEndpointDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc9fed81_10ca_4533_952d_9e278341e80f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbInterruptOutEndpointDescriptorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbInterruptOutPipe(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbInterruptOutPipe {
    type Vtable = IUsbInterruptOutPipeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe984c8a9_aaf9_49d0_b96c_f661ab4a7f95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbInterruptOutPipeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UsbWriteOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UsbWriteOptions) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbSetupPacket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbSetupPacket {
    type Vtable = IUsbSetupPacketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x104ba132_c78f_4c51_b654_e49d02f2cb03);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbSetupPacketVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IUsbSetupPacketFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUsbSetupPacketFactory {
    type Vtable = IUsbSetupPacketFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9257d50_1b2e_4a41_a2a7_338f0cef3c14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUsbSetupPacketFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eightbytebuffer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbBulkInEndpointDescriptor(::windows::core::IUnknown);
impl UsbBulkInEndpointDescriptor {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn MaxPacketSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn EndpointNumber(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Pipe(&self) -> ::windows::core::Result<UsbBulkInPipe> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbBulkInPipe>(result__)
        }
    }
}
impl ::core::clone::Clone for UsbBulkInEndpointDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbBulkInEndpointDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbBulkInEndpointDescriptor {}
impl ::core::fmt::Debug for UsbBulkInEndpointDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbBulkInEndpointDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbBulkInEndpointDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbBulkInEndpointDescriptor;{3c6e4846-06cf-42a9-9dc2-971c1b14b6e3})");
}
unsafe impl ::windows::core::Interface for UsbBulkInEndpointDescriptor {
    type Vtable = IUsbBulkInEndpointDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c6e4846_06cf_42a9_9dc2_971c1b14b6e3);
}
impl ::windows::core::RuntimeName for UsbBulkInEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbBulkInEndpointDescriptor";
}
impl ::core::convert::From<UsbBulkInEndpointDescriptor> for ::windows::core::IUnknown {
    fn from(value: UsbBulkInEndpointDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbBulkInEndpointDescriptor> for ::windows::core::IUnknown {
    fn from(value: &UsbBulkInEndpointDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbBulkInEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbBulkInEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbBulkInEndpointDescriptor> for ::windows::core::IInspectable {
    fn from(value: UsbBulkInEndpointDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbBulkInEndpointDescriptor> for ::windows::core::IInspectable {
    fn from(value: &UsbBulkInEndpointDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbBulkInEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbBulkInEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbBulkInEndpointDescriptor {}
unsafe impl ::core::marker::Sync for UsbBulkInEndpointDescriptor {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbBulkInPipe(::windows::core::IUnknown);
impl UsbBulkInPipe {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn MaxTransferSizeBytes(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn EndpointDescriptor(&self) -> ::windows::core::Result<UsbBulkInEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbBulkInEndpointDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearStallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetReadOptions(&self, value: UsbReadOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn ReadOptions(&self) -> ::windows::core::Result<UsbReadOptions> {
        let this = self;
        unsafe {
            let mut result__: UsbReadOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbReadOptions>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn FlushBuffer(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
}
impl ::core::clone::Clone for UsbBulkInPipe {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbBulkInPipe {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbBulkInPipe {}
impl ::core::fmt::Debug for UsbBulkInPipe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbBulkInPipe").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbBulkInPipe {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbBulkInPipe;{f01d2d3b-4548-4d50-b326-d82cdabe1220})");
}
unsafe impl ::windows::core::Interface for UsbBulkInPipe {
    type Vtable = IUsbBulkInPipeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf01d2d3b_4548_4d50_b326_d82cdabe1220);
}
impl ::windows::core::RuntimeName for UsbBulkInPipe {
    const NAME: &'static str = "Windows.Devices.Usb.UsbBulkInPipe";
}
impl ::core::convert::From<UsbBulkInPipe> for ::windows::core::IUnknown {
    fn from(value: UsbBulkInPipe) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbBulkInPipe> for ::windows::core::IUnknown {
    fn from(value: &UsbBulkInPipe) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbBulkInPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbBulkInPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbBulkInPipe> for ::windows::core::IInspectable {
    fn from(value: UsbBulkInPipe) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbBulkInPipe> for ::windows::core::IInspectable {
    fn from(value: &UsbBulkInPipe) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbBulkInPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbBulkInPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbBulkInPipe {}
unsafe impl ::core::marker::Sync for UsbBulkInPipe {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbBulkOutEndpointDescriptor(::windows::core::IUnknown);
impl UsbBulkOutEndpointDescriptor {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn MaxPacketSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn EndpointNumber(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Pipe(&self) -> ::windows::core::Result<UsbBulkOutPipe> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbBulkOutPipe>(result__)
        }
    }
}
impl ::core::clone::Clone for UsbBulkOutEndpointDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbBulkOutEndpointDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbBulkOutEndpointDescriptor {}
impl ::core::fmt::Debug for UsbBulkOutEndpointDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbBulkOutEndpointDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbBulkOutEndpointDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbBulkOutEndpointDescriptor;{2820847a-ffee-4f60-9be1-956cac3ecb65})");
}
unsafe impl ::windows::core::Interface for UsbBulkOutEndpointDescriptor {
    type Vtable = IUsbBulkOutEndpointDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2820847a_ffee_4f60_9be1_956cac3ecb65);
}
impl ::windows::core::RuntimeName for UsbBulkOutEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbBulkOutEndpointDescriptor";
}
impl ::core::convert::From<UsbBulkOutEndpointDescriptor> for ::windows::core::IUnknown {
    fn from(value: UsbBulkOutEndpointDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbBulkOutEndpointDescriptor> for ::windows::core::IUnknown {
    fn from(value: &UsbBulkOutEndpointDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbBulkOutEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbBulkOutEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbBulkOutEndpointDescriptor> for ::windows::core::IInspectable {
    fn from(value: UsbBulkOutEndpointDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbBulkOutEndpointDescriptor> for ::windows::core::IInspectable {
    fn from(value: &UsbBulkOutEndpointDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbBulkOutEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbBulkOutEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbBulkOutEndpointDescriptor {}
unsafe impl ::core::marker::Sync for UsbBulkOutEndpointDescriptor {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbBulkOutPipe(::windows::core::IUnknown);
impl UsbBulkOutPipe {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn EndpointDescriptor(&self) -> ::windows::core::Result<UsbBulkOutEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbBulkOutEndpointDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearStallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetWriteOptions(&self, value: UsbWriteOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn WriteOptions(&self) -> ::windows::core::Result<UsbWriteOptions> {
        let this = self;
        unsafe {
            let mut result__: UsbWriteOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbWriteOptions>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
}
impl ::core::clone::Clone for UsbBulkOutPipe {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbBulkOutPipe {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbBulkOutPipe {}
impl ::core::fmt::Debug for UsbBulkOutPipe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbBulkOutPipe").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbBulkOutPipe {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbBulkOutPipe;{a8e9ee6e-0115-45aa-8b21-37b225bccee7})");
}
unsafe impl ::windows::core::Interface for UsbBulkOutPipe {
    type Vtable = IUsbBulkOutPipeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8e9ee6e_0115_45aa_8b21_37b225bccee7);
}
impl ::windows::core::RuntimeName for UsbBulkOutPipe {
    const NAME: &'static str = "Windows.Devices.Usb.UsbBulkOutPipe";
}
impl ::core::convert::From<UsbBulkOutPipe> for ::windows::core::IUnknown {
    fn from(value: UsbBulkOutPipe) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbBulkOutPipe> for ::windows::core::IUnknown {
    fn from(value: &UsbBulkOutPipe) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbBulkOutPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbBulkOutPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbBulkOutPipe> for ::windows::core::IInspectable {
    fn from(value: UsbBulkOutPipe) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbBulkOutPipe> for ::windows::core::IInspectable {
    fn from(value: &UsbBulkOutPipe) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbBulkOutPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbBulkOutPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbBulkOutPipe {}
unsafe impl ::core::marker::Sync for UsbBulkOutPipe {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbConfiguration(::windows::core::IUnknown);
impl UsbConfiguration {
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UsbInterfaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterface>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbInterface>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn ConfigurationDescriptor(&self) -> ::windows::core::Result<UsbConfigurationDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbConfigurationDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Descriptors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbDescriptor>>(result__)
        }
    }
}
impl ::core::clone::Clone for UsbConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbConfiguration {}
impl ::core::fmt::Debug for UsbConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbConfiguration;{68177429-36a9-46d7-b873-fc689251ec30})");
}
unsafe impl ::windows::core::Interface for UsbConfiguration {
    type Vtable = IUsbConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68177429_36a9_46d7_b873_fc689251ec30);
}
impl ::windows::core::RuntimeName for UsbConfiguration {
    const NAME: &'static str = "Windows.Devices.Usb.UsbConfiguration";
}
impl ::core::convert::From<UsbConfiguration> for ::windows::core::IUnknown {
    fn from(value: UsbConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbConfiguration> for ::windows::core::IUnknown {
    fn from(value: &UsbConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbConfiguration> for ::windows::core::IInspectable {
    fn from(value: UsbConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbConfiguration> for ::windows::core::IInspectable {
    fn from(value: &UsbConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbConfiguration {}
unsafe impl ::core::marker::Sync for UsbConfiguration {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbConfigurationDescriptor(::windows::core::IUnknown);
impl UsbConfigurationDescriptor {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn ConfigurationValue(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn MaxPowerMilliamps(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SelfPowered(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn RemoteWakeup(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, UsbDescriptor>>(descriptor: Param0, parsed: &mut ::core::option::Option<UsbConfigurationDescriptor>) -> ::windows::core::Result<bool> {
        Self::IUsbConfigurationDescriptorStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), descriptor.into_param().abi(), parsed as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, UsbDescriptor>>(descriptor: Param0) -> ::windows::core::Result<UsbConfigurationDescriptor> {
        Self::IUsbConfigurationDescriptorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), descriptor.into_param().abi(), &mut result__).from_abi::<UsbConfigurationDescriptor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUsbConfigurationDescriptorStatics<R, F: FnOnce(&IUsbConfigurationDescriptorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UsbConfigurationDescriptor, IUsbConfigurationDescriptorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UsbConfigurationDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbConfigurationDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbConfigurationDescriptor {}
impl ::core::fmt::Debug for UsbConfigurationDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbConfigurationDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbConfigurationDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbConfigurationDescriptor;{f2176d92-b442-407a-8207-7d646c0385f3})");
}
unsafe impl ::windows::core::Interface for UsbConfigurationDescriptor {
    type Vtable = IUsbConfigurationDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2176d92_b442_407a_8207_7d646c0385f3);
}
impl ::windows::core::RuntimeName for UsbConfigurationDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbConfigurationDescriptor";
}
impl ::core::convert::From<UsbConfigurationDescriptor> for ::windows::core::IUnknown {
    fn from(value: UsbConfigurationDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbConfigurationDescriptor> for ::windows::core::IUnknown {
    fn from(value: &UsbConfigurationDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbConfigurationDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbConfigurationDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbConfigurationDescriptor> for ::windows::core::IInspectable {
    fn from(value: UsbConfigurationDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbConfigurationDescriptor> for ::windows::core::IInspectable {
    fn from(value: &UsbConfigurationDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbConfigurationDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbConfigurationDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbConfigurationDescriptor {}
unsafe impl ::core::marker::Sync for UsbConfigurationDescriptor {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UsbControlRecipient(pub i32);
impl UsbControlRecipient {
    pub const Device: Self = Self(0i32);
    pub const SpecifiedInterface: Self = Self(1i32);
    pub const Endpoint: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
    pub const DefaultInterface: Self = Self(4i32);
}
impl ::core::marker::Copy for UsbControlRecipient {}
impl ::core::clone::Clone for UsbControlRecipient {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UsbControlRecipient {
    type Abi = Self;
}
impl ::core::fmt::Debug for UsbControlRecipient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbControlRecipient").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbControlRecipient {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Usb.UsbControlRecipient;i4)");
}
impl ::windows::core::DefaultType for UsbControlRecipient {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbControlRequestType(::windows::core::IUnknown);
impl UsbControlRequestType {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UsbControlRequestType, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Direction(&self) -> ::windows::core::Result<UsbTransferDirection> {
        let this = self;
        unsafe {
            let mut result__: UsbTransferDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbTransferDirection>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetDirection(&self, value: UsbTransferDirection) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn ControlTransferType(&self) -> ::windows::core::Result<UsbControlTransferType> {
        let this = self;
        unsafe {
            let mut result__: UsbControlTransferType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbControlTransferType>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetControlTransferType(&self, value: UsbControlTransferType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Recipient(&self) -> ::windows::core::Result<UsbControlRecipient> {
        let this = self;
        unsafe {
            let mut result__: UsbControlRecipient = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbControlRecipient>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetRecipient(&self, value: UsbControlRecipient) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn AsByte(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetAsByte(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for UsbControlRequestType {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbControlRequestType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbControlRequestType {}
impl ::core::fmt::Debug for UsbControlRequestType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbControlRequestType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbControlRequestType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbControlRequestType;{8e9465a6-d73d-46de-94be-aae7f07c0f5c})");
}
unsafe impl ::windows::core::Interface for UsbControlRequestType {
    type Vtable = IUsbControlRequestTypeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e9465a6_d73d_46de_94be_aae7f07c0f5c);
}
impl ::windows::core::RuntimeName for UsbControlRequestType {
    const NAME: &'static str = "Windows.Devices.Usb.UsbControlRequestType";
}
impl ::core::convert::From<UsbControlRequestType> for ::windows::core::IUnknown {
    fn from(value: UsbControlRequestType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbControlRequestType> for ::windows::core::IUnknown {
    fn from(value: &UsbControlRequestType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbControlRequestType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbControlRequestType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbControlRequestType> for ::windows::core::IInspectable {
    fn from(value: UsbControlRequestType) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbControlRequestType> for ::windows::core::IInspectable {
    fn from(value: &UsbControlRequestType) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbControlRequestType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbControlRequestType {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbControlRequestType {}
unsafe impl ::core::marker::Sync for UsbControlRequestType {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UsbControlTransferType(pub i32);
impl UsbControlTransferType {
    pub const Standard: Self = Self(0i32);
    pub const Class: Self = Self(1i32);
    pub const Vendor: Self = Self(2i32);
}
impl ::core::marker::Copy for UsbControlTransferType {}
impl ::core::clone::Clone for UsbControlTransferType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UsbControlTransferType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UsbControlTransferType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbControlTransferType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbControlTransferType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Usb.UsbControlTransferType;i4)");
}
impl ::windows::core::DefaultType for UsbControlTransferType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbDescriptor(::windows::core::IUnknown);
impl UsbDescriptor {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Length(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn DescriptorType(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ReadDescriptorBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), buffer.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UsbDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbDescriptor {}
impl ::core::fmt::Debug for UsbDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbDescriptor;{0a89f216-5f9d-4874-8904-da9ad3f5528f})");
}
unsafe impl ::windows::core::Interface for UsbDescriptor {
    type Vtable = IUsbDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a89f216_5f9d_4874_8904_da9ad3f5528f);
}
impl ::windows::core::RuntimeName for UsbDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbDescriptor";
}
impl ::core::convert::From<UsbDescriptor> for ::windows::core::IUnknown {
    fn from(value: UsbDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbDescriptor> for ::windows::core::IUnknown {
    fn from(value: &UsbDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbDescriptor> for ::windows::core::IInspectable {
    fn from(value: UsbDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbDescriptor> for ::windows::core::IInspectable {
    fn from(value: &UsbDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbDescriptor {}
unsafe impl ::core::marker::Sync for UsbDescriptor {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbDevice(::windows::core::IUnknown);
impl UsbDevice {
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendControlOutTransferAsync<'a, Param0: ::windows::core::IntoParam<'a, UsbSetupPacket>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, setuppacket: Param0, buffer: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), setuppacket.into_param().abi(), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SendControlOutTransferAsyncNoBuffer<'a, Param0: ::windows::core::IntoParam<'a, UsbSetupPacket>>(&self, setuppacket: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), setuppacket.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendControlInTransferAsync<'a, Param0: ::windows::core::IntoParam<'a, UsbSetupPacket>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, setuppacket: Param0, buffer: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), setuppacket.into_param().abi(), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendControlInTransferAsyncNoBuffer<'a, Param0: ::windows::core::IntoParam<'a, UsbSetupPacket>>(&self, setuppacket: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), setuppacket.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn DefaultInterface(&self) -> ::windows::core::Result<UsbInterface> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbInterface>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn DeviceDescriptor(&self) -> ::windows::core::Result<UsbDeviceDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbDeviceDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Configuration(&self) -> ::windows::core::Result<UsbConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbConfiguration>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn GetDeviceSelector<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(vendorid: u32, productid: u32, winusbinterfaceclass: Param2) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IUsbDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), vendorid, productid, winusbinterfaceclass.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn GetDeviceSelectorGuidOnly<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(winusbinterfaceclass: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IUsbDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), winusbinterfaceclass.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn GetDeviceSelectorVidPidOnly(vendorid: u32, productid: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IUsbDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), vendorid, productid, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn GetDeviceClassSelector<'a, Param0: ::windows::core::IntoParam<'a, UsbDeviceClass>>(usbclass: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IUsbDeviceStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), usbclass.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UsbDevice>> {
        Self::IUsbDeviceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UsbDevice>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUsbDeviceStatics<R, F: FnOnce(&IUsbDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UsbDevice, IUsbDeviceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UsbDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbDevice {}
impl ::core::fmt::Debug for UsbDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbDevice;{5249b992-c456-44d5-ad5e-24f5a089f63b})");
}
unsafe impl ::windows::core::Interface for UsbDevice {
    type Vtable = IUsbDeviceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5249b992_c456_44d5_ad5e_24f5a089f63b);
}
impl ::windows::core::RuntimeName for UsbDevice {
    const NAME: &'static str = "Windows.Devices.Usb.UsbDevice";
}
impl ::core::convert::From<UsbDevice> for ::windows::core::IUnknown {
    fn from(value: UsbDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbDevice> for ::windows::core::IUnknown {
    fn from(value: &UsbDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbDevice> for ::windows::core::IInspectable {
    fn from(value: UsbDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbDevice> for ::windows::core::IInspectable {
    fn from(value: &UsbDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<UsbDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: UsbDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&UsbDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &UsbDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for UsbDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &UsbDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UsbDevice {}
unsafe impl ::core::marker::Sync for UsbDevice {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbDeviceClass(::windows::core::IUnknown);
impl UsbDeviceClass {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UsbDeviceClass, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn ClassCode(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetClassCode(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SubclassCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u8>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSubclassCode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<u8>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ProtocolCode(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<u8>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetProtocolCode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<u8>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UsbDeviceClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbDeviceClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbDeviceClass {}
impl ::core::fmt::Debug for UsbDeviceClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbDeviceClass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbDeviceClass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbDeviceClass;{051942f9-845e-47eb-b12a-38f2f617afe7})");
}
unsafe impl ::windows::core::Interface for UsbDeviceClass {
    type Vtable = IUsbDeviceClassVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x051942f9_845e_47eb_b12a_38f2f617afe7);
}
impl ::windows::core::RuntimeName for UsbDeviceClass {
    const NAME: &'static str = "Windows.Devices.Usb.UsbDeviceClass";
}
impl ::core::convert::From<UsbDeviceClass> for ::windows::core::IUnknown {
    fn from(value: UsbDeviceClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbDeviceClass> for ::windows::core::IUnknown {
    fn from(value: &UsbDeviceClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbDeviceClass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbDeviceClass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbDeviceClass> for ::windows::core::IInspectable {
    fn from(value: UsbDeviceClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbDeviceClass> for ::windows::core::IInspectable {
    fn from(value: &UsbDeviceClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbDeviceClass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbDeviceClass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbDeviceClass {}
unsafe impl ::core::marker::Sync for UsbDeviceClass {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbDeviceClasses(::windows::core::IUnknown);
impl UsbDeviceClasses {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn CdcControl() -> ::windows::core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbDeviceClass>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Physical() -> ::windows::core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbDeviceClass>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn PersonalHealthcare() -> ::windows::core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbDeviceClass>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn ActiveSync() -> ::windows::core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbDeviceClass>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn PalmSync() -> ::windows::core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbDeviceClass>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn DeviceFirmwareUpdate() -> ::windows::core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbDeviceClass>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Irda() -> ::windows::core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbDeviceClass>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Measurement() -> ::windows::core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbDeviceClass>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn VendorSpecific() -> ::windows::core::Result<UsbDeviceClass> {
        Self::IUsbDeviceClassesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbDeviceClass>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUsbDeviceClassesStatics<R, F: FnOnce(&IUsbDeviceClassesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UsbDeviceClasses, IUsbDeviceClassesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UsbDeviceClasses {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbDeviceClasses {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbDeviceClasses {}
impl ::core::fmt::Debug for UsbDeviceClasses {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbDeviceClasses").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbDeviceClasses {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbDeviceClasses;{686f955d-9b92-4b30-9781-c22c55ac35cb})");
}
unsafe impl ::windows::core::Interface for UsbDeviceClasses {
    type Vtable = IUsbDeviceClassesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x686f955d_9b92_4b30_9781_c22c55ac35cb);
}
impl ::windows::core::RuntimeName for UsbDeviceClasses {
    const NAME: &'static str = "Windows.Devices.Usb.UsbDeviceClasses";
}
impl ::core::convert::From<UsbDeviceClasses> for ::windows::core::IUnknown {
    fn from(value: UsbDeviceClasses) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbDeviceClasses> for ::windows::core::IUnknown {
    fn from(value: &UsbDeviceClasses) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbDeviceClasses {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbDeviceClasses {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbDeviceClasses> for ::windows::core::IInspectable {
    fn from(value: UsbDeviceClasses) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbDeviceClasses> for ::windows::core::IInspectable {
    fn from(value: &UsbDeviceClasses) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbDeviceClasses {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbDeviceClasses {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbDeviceClasses {}
unsafe impl ::core::marker::Sync for UsbDeviceClasses {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbDeviceDescriptor(::windows::core::IUnknown);
impl UsbDeviceDescriptor {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn BcdUsb(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn MaxPacketSize0(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn VendorId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn ProductId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn BcdDeviceRevision(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn NumberOfConfigurations(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
}
impl ::core::clone::Clone for UsbDeviceDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbDeviceDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbDeviceDescriptor {}
impl ::core::fmt::Debug for UsbDeviceDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbDeviceDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbDeviceDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbDeviceDescriptor;{1f48d1f6-ba97-4322-b92c-b5b189216588})");
}
unsafe impl ::windows::core::Interface for UsbDeviceDescriptor {
    type Vtable = IUsbDeviceDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f48d1f6_ba97_4322_b92c_b5b189216588);
}
impl ::windows::core::RuntimeName for UsbDeviceDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbDeviceDescriptor";
}
impl ::core::convert::From<UsbDeviceDescriptor> for ::windows::core::IUnknown {
    fn from(value: UsbDeviceDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbDeviceDescriptor> for ::windows::core::IUnknown {
    fn from(value: &UsbDeviceDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbDeviceDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbDeviceDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbDeviceDescriptor> for ::windows::core::IInspectable {
    fn from(value: UsbDeviceDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbDeviceDescriptor> for ::windows::core::IInspectable {
    fn from(value: &UsbDeviceDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbDeviceDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbDeviceDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbDeviceDescriptor {}
unsafe impl ::core::marker::Sync for UsbDeviceDescriptor {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbEndpointDescriptor(::windows::core::IUnknown);
impl UsbEndpointDescriptor {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn EndpointNumber(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Direction(&self) -> ::windows::core::Result<UsbTransferDirection> {
        let this = self;
        unsafe {
            let mut result__: UsbTransferDirection = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbTransferDirection>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn EndpointType(&self) -> ::windows::core::Result<UsbEndpointType> {
        let this = self;
        unsafe {
            let mut result__: UsbEndpointType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbEndpointType>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn AsBulkInEndpointDescriptor(&self) -> ::windows::core::Result<UsbBulkInEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbBulkInEndpointDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn AsInterruptInEndpointDescriptor(&self) -> ::windows::core::Result<UsbInterruptInEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbInterruptInEndpointDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn AsBulkOutEndpointDescriptor(&self) -> ::windows::core::Result<UsbBulkOutEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbBulkOutEndpointDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn AsInterruptOutEndpointDescriptor(&self) -> ::windows::core::Result<UsbInterruptOutEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbInterruptOutEndpointDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, UsbDescriptor>>(descriptor: Param0, parsed: &mut ::core::option::Option<UsbEndpointDescriptor>) -> ::windows::core::Result<bool> {
        Self::IUsbEndpointDescriptorStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), descriptor.into_param().abi(), parsed as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, UsbDescriptor>>(descriptor: Param0) -> ::windows::core::Result<UsbEndpointDescriptor> {
        Self::IUsbEndpointDescriptorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), descriptor.into_param().abi(), &mut result__).from_abi::<UsbEndpointDescriptor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUsbEndpointDescriptorStatics<R, F: FnOnce(&IUsbEndpointDescriptorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UsbEndpointDescriptor, IUsbEndpointDescriptorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UsbEndpointDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbEndpointDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbEndpointDescriptor {}
impl ::core::fmt::Debug for UsbEndpointDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbEndpointDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbEndpointDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbEndpointDescriptor;{6b4862d9-8df7-4b40-ac83-578f139f0575})");
}
unsafe impl ::windows::core::Interface for UsbEndpointDescriptor {
    type Vtable = IUsbEndpointDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b4862d9_8df7_4b40_ac83_578f139f0575);
}
impl ::windows::core::RuntimeName for UsbEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbEndpointDescriptor";
}
impl ::core::convert::From<UsbEndpointDescriptor> for ::windows::core::IUnknown {
    fn from(value: UsbEndpointDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbEndpointDescriptor> for ::windows::core::IUnknown {
    fn from(value: &UsbEndpointDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbEndpointDescriptor> for ::windows::core::IInspectable {
    fn from(value: UsbEndpointDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbEndpointDescriptor> for ::windows::core::IInspectable {
    fn from(value: &UsbEndpointDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbEndpointDescriptor {}
unsafe impl ::core::marker::Sync for UsbEndpointDescriptor {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UsbEndpointType(pub i32);
impl UsbEndpointType {
    pub const Control: Self = Self(0i32);
    pub const Isochronous: Self = Self(1i32);
    pub const Bulk: Self = Self(2i32);
    pub const Interrupt: Self = Self(3i32);
}
impl ::core::marker::Copy for UsbEndpointType {}
impl ::core::clone::Clone for UsbEndpointType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UsbEndpointType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UsbEndpointType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbEndpointType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbEndpointType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Usb.UsbEndpointType;i4)");
}
impl ::windows::core::DefaultType for UsbEndpointType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbInterface(::windows::core::IUnknown);
impl UsbInterface {
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BulkInPipes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkInPipe>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbBulkInPipe>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InterruptInPipes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptInPipe>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbInterruptInPipe>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BulkOutPipes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkOutPipe>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbBulkOutPipe>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InterruptOutPipes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptOutPipe>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbInterruptOutPipe>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InterfaceSettings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterfaceSetting>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbInterfaceSetting>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn InterfaceNumber(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Descriptors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbDescriptor>>(result__)
        }
    }
}
impl ::core::clone::Clone for UsbInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbInterface {}
impl ::core::fmt::Debug for UsbInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbInterface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbInterface {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbInterface;{a0322b95-7f47-48ab-a727-678c25be2112})");
}
unsafe impl ::windows::core::Interface for UsbInterface {
    type Vtable = IUsbInterfaceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa0322b95_7f47_48ab_a727_678c25be2112);
}
impl ::windows::core::RuntimeName for UsbInterface {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterface";
}
impl ::core::convert::From<UsbInterface> for ::windows::core::IUnknown {
    fn from(value: UsbInterface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterface> for ::windows::core::IUnknown {
    fn from(value: &UsbInterface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbInterface> for ::windows::core::IInspectable {
    fn from(value: UsbInterface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterface> for ::windows::core::IInspectable {
    fn from(value: &UsbInterface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbInterface {}
unsafe impl ::core::marker::Sync for UsbInterface {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbInterfaceDescriptor(::windows::core::IUnknown);
impl UsbInterfaceDescriptor {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn ClassCode(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SubclassCode(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn ProtocolCode(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn AlternateSettingNumber(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn InterfaceNumber(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn TryParse<'a, Param0: ::windows::core::IntoParam<'a, UsbDescriptor>>(descriptor: Param0, parsed: &mut ::core::option::Option<UsbInterfaceDescriptor>) -> ::windows::core::Result<bool> {
        Self::IUsbInterfaceDescriptorStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), descriptor.into_param().abi(), parsed as *mut _ as _, &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Parse<'a, Param0: ::windows::core::IntoParam<'a, UsbDescriptor>>(descriptor: Param0) -> ::windows::core::Result<UsbInterfaceDescriptor> {
        Self::IUsbInterfaceDescriptorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), descriptor.into_param().abi(), &mut result__).from_abi::<UsbInterfaceDescriptor>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUsbInterfaceDescriptorStatics<R, F: FnOnce(&IUsbInterfaceDescriptorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UsbInterfaceDescriptor, IUsbInterfaceDescriptorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UsbInterfaceDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbInterfaceDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbInterfaceDescriptor {}
impl ::core::fmt::Debug for UsbInterfaceDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbInterfaceDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbInterfaceDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbInterfaceDescriptor;{199670c7-b7ee-4f90-8cd5-94a2e257598a})");
}
unsafe impl ::windows::core::Interface for UsbInterfaceDescriptor {
    type Vtable = IUsbInterfaceDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x199670c7_b7ee_4f90_8cd5_94a2e257598a);
}
impl ::windows::core::RuntimeName for UsbInterfaceDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterfaceDescriptor";
}
impl ::core::convert::From<UsbInterfaceDescriptor> for ::windows::core::IUnknown {
    fn from(value: UsbInterfaceDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterfaceDescriptor> for ::windows::core::IUnknown {
    fn from(value: &UsbInterfaceDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbInterfaceDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbInterfaceDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbInterfaceDescriptor> for ::windows::core::IInspectable {
    fn from(value: UsbInterfaceDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterfaceDescriptor> for ::windows::core::IInspectable {
    fn from(value: &UsbInterfaceDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbInterfaceDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbInterfaceDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbInterfaceDescriptor {}
unsafe impl ::core::marker::Sync for UsbInterfaceDescriptor {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbInterfaceSetting(::windows::core::IUnknown);
impl UsbInterfaceSetting {
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BulkInEndpoints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkInEndpointDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbBulkInEndpointDescriptor>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InterruptInEndpoints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptInEndpointDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbInterruptInEndpointDescriptor>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BulkOutEndpoints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbBulkOutEndpointDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbBulkOutEndpointDescriptor>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InterruptOutEndpoints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbInterruptOutEndpointDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbInterruptOutEndpointDescriptor>>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Selected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectSettingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn InterfaceDescriptor(&self) -> ::windows::core::Result<UsbInterfaceDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbInterfaceDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Descriptors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UsbDescriptor>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UsbDescriptor>>(result__)
        }
    }
}
impl ::core::clone::Clone for UsbInterfaceSetting {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbInterfaceSetting {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbInterfaceSetting {}
impl ::core::fmt::Debug for UsbInterfaceSetting {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbInterfaceSetting").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbInterfaceSetting {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbInterfaceSetting;{1827bba7-8da7-4af7-8f4c-7f3032e781f5})");
}
unsafe impl ::windows::core::Interface for UsbInterfaceSetting {
    type Vtable = IUsbInterfaceSettingVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1827bba7_8da7_4af7_8f4c_7f3032e781f5);
}
impl ::windows::core::RuntimeName for UsbInterfaceSetting {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterfaceSetting";
}
impl ::core::convert::From<UsbInterfaceSetting> for ::windows::core::IUnknown {
    fn from(value: UsbInterfaceSetting) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterfaceSetting> for ::windows::core::IUnknown {
    fn from(value: &UsbInterfaceSetting) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbInterfaceSetting {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbInterfaceSetting {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbInterfaceSetting> for ::windows::core::IInspectable {
    fn from(value: UsbInterfaceSetting) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterfaceSetting> for ::windows::core::IInspectable {
    fn from(value: &UsbInterfaceSetting) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbInterfaceSetting {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbInterfaceSetting {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbInterfaceSetting {}
unsafe impl ::core::marker::Sync for UsbInterfaceSetting {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbInterruptInEndpointDescriptor(::windows::core::IUnknown);
impl UsbInterruptInEndpointDescriptor {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn MaxPacketSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn EndpointNumber(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Interval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Pipe(&self) -> ::windows::core::Result<UsbInterruptInPipe> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbInterruptInPipe>(result__)
        }
    }
}
impl ::core::clone::Clone for UsbInterruptInEndpointDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbInterruptInEndpointDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbInterruptInEndpointDescriptor {}
impl ::core::fmt::Debug for UsbInterruptInEndpointDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbInterruptInEndpointDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbInterruptInEndpointDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbInterruptInEndpointDescriptor;{c0528967-c911-4c3a-86b2-419c2da89039})");
}
unsafe impl ::windows::core::Interface for UsbInterruptInEndpointDescriptor {
    type Vtable = IUsbInterruptInEndpointDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0528967_c911_4c3a_86b2_419c2da89039);
}
impl ::windows::core::RuntimeName for UsbInterruptInEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterruptInEndpointDescriptor";
}
impl ::core::convert::From<UsbInterruptInEndpointDescriptor> for ::windows::core::IUnknown {
    fn from(value: UsbInterruptInEndpointDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterruptInEndpointDescriptor> for ::windows::core::IUnknown {
    fn from(value: &UsbInterruptInEndpointDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbInterruptInEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbInterruptInEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbInterruptInEndpointDescriptor> for ::windows::core::IInspectable {
    fn from(value: UsbInterruptInEndpointDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterruptInEndpointDescriptor> for ::windows::core::IInspectable {
    fn from(value: &UsbInterruptInEndpointDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbInterruptInEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbInterruptInEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbInterruptInEndpointDescriptor {}
unsafe impl ::core::marker::Sync for UsbInterruptInEndpointDescriptor {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbInterruptInEventArgs(::windows::core::IUnknown);
impl UsbInterruptInEventArgs {
    #[doc = "*Required features: 'Devices_Usb', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn InterruptData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for UsbInterruptInEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbInterruptInEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbInterruptInEventArgs {}
impl ::core::fmt::Debug for UsbInterruptInEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbInterruptInEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbInterruptInEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbInterruptInEventArgs;{b7b04092-1418-4936-8209-299cf5605583})");
}
unsafe impl ::windows::core::Interface for UsbInterruptInEventArgs {
    type Vtable = IUsbInterruptInEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7b04092_1418_4936_8209_299cf5605583);
}
impl ::windows::core::RuntimeName for UsbInterruptInEventArgs {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterruptInEventArgs";
}
impl ::core::convert::From<UsbInterruptInEventArgs> for ::windows::core::IUnknown {
    fn from(value: UsbInterruptInEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterruptInEventArgs> for ::windows::core::IUnknown {
    fn from(value: &UsbInterruptInEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbInterruptInEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbInterruptInEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbInterruptInEventArgs> for ::windows::core::IInspectable {
    fn from(value: UsbInterruptInEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterruptInEventArgs> for ::windows::core::IInspectable {
    fn from(value: &UsbInterruptInEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbInterruptInEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbInterruptInEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbInterruptInEventArgs {}
unsafe impl ::core::marker::Sync for UsbInterruptInEventArgs {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbInterruptInPipe(::windows::core::IUnknown);
impl UsbInterruptInPipe {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn EndpointDescriptor(&self) -> ::windows::core::Result<UsbInterruptInEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbInterruptInEndpointDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearStallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<UsbInterruptInPipe, UsbInterruptInEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UsbInterruptInPipe {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbInterruptInPipe {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbInterruptInPipe {}
impl ::core::fmt::Debug for UsbInterruptInPipe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbInterruptInPipe").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbInterruptInPipe {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbInterruptInPipe;{fa007116-84d7-48c7-8a3f-4c0b235f2ea6})");
}
unsafe impl ::windows::core::Interface for UsbInterruptInPipe {
    type Vtable = IUsbInterruptInPipeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa007116_84d7_48c7_8a3f_4c0b235f2ea6);
}
impl ::windows::core::RuntimeName for UsbInterruptInPipe {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterruptInPipe";
}
impl ::core::convert::From<UsbInterruptInPipe> for ::windows::core::IUnknown {
    fn from(value: UsbInterruptInPipe) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterruptInPipe> for ::windows::core::IUnknown {
    fn from(value: &UsbInterruptInPipe) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbInterruptInPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbInterruptInPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbInterruptInPipe> for ::windows::core::IInspectable {
    fn from(value: UsbInterruptInPipe) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterruptInPipe> for ::windows::core::IInspectable {
    fn from(value: &UsbInterruptInPipe) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbInterruptInPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbInterruptInPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbInterruptInPipe {}
unsafe impl ::core::marker::Sync for UsbInterruptInPipe {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbInterruptOutEndpointDescriptor(::windows::core::IUnknown);
impl UsbInterruptOutEndpointDescriptor {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn MaxPacketSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn EndpointNumber(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Interval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Pipe(&self) -> ::windows::core::Result<UsbInterruptOutPipe> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbInterruptOutPipe>(result__)
        }
    }
}
impl ::core::clone::Clone for UsbInterruptOutEndpointDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbInterruptOutEndpointDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbInterruptOutEndpointDescriptor {}
impl ::core::fmt::Debug for UsbInterruptOutEndpointDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbInterruptOutEndpointDescriptor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbInterruptOutEndpointDescriptor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbInterruptOutEndpointDescriptor;{cc9fed81-10ca-4533-952d-9e278341e80f})");
}
unsafe impl ::windows::core::Interface for UsbInterruptOutEndpointDescriptor {
    type Vtable = IUsbInterruptOutEndpointDescriptorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc9fed81_10ca_4533_952d_9e278341e80f);
}
impl ::windows::core::RuntimeName for UsbInterruptOutEndpointDescriptor {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterruptOutEndpointDescriptor";
}
impl ::core::convert::From<UsbInterruptOutEndpointDescriptor> for ::windows::core::IUnknown {
    fn from(value: UsbInterruptOutEndpointDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterruptOutEndpointDescriptor> for ::windows::core::IUnknown {
    fn from(value: &UsbInterruptOutEndpointDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbInterruptOutEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbInterruptOutEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbInterruptOutEndpointDescriptor> for ::windows::core::IInspectable {
    fn from(value: UsbInterruptOutEndpointDescriptor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterruptOutEndpointDescriptor> for ::windows::core::IInspectable {
    fn from(value: &UsbInterruptOutEndpointDescriptor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbInterruptOutEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbInterruptOutEndpointDescriptor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbInterruptOutEndpointDescriptor {}
unsafe impl ::core::marker::Sync for UsbInterruptOutEndpointDescriptor {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbInterruptOutPipe(::windows::core::IUnknown);
impl UsbInterruptOutPipe {
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn EndpointDescriptor(&self) -> ::windows::core::Result<UsbInterruptOutEndpointDescriptor> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbInterruptOutEndpointDescriptor>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearStallAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetWriteOptions(&self, value: UsbWriteOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn WriteOptions(&self) -> ::windows::core::Result<UsbWriteOptions> {
        let this = self;
        unsafe {
            let mut result__: UsbWriteOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbWriteOptions>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
}
impl ::core::clone::Clone for UsbInterruptOutPipe {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbInterruptOutPipe {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbInterruptOutPipe {}
impl ::core::fmt::Debug for UsbInterruptOutPipe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbInterruptOutPipe").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbInterruptOutPipe {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbInterruptOutPipe;{e984c8a9-aaf9-49d0-b96c-f661ab4a7f95})");
}
unsafe impl ::windows::core::Interface for UsbInterruptOutPipe {
    type Vtable = IUsbInterruptOutPipeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe984c8a9_aaf9_49d0_b96c_f661ab4a7f95);
}
impl ::windows::core::RuntimeName for UsbInterruptOutPipe {
    const NAME: &'static str = "Windows.Devices.Usb.UsbInterruptOutPipe";
}
impl ::core::convert::From<UsbInterruptOutPipe> for ::windows::core::IUnknown {
    fn from(value: UsbInterruptOutPipe) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterruptOutPipe> for ::windows::core::IUnknown {
    fn from(value: &UsbInterruptOutPipe) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbInterruptOutPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbInterruptOutPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbInterruptOutPipe> for ::windows::core::IInspectable {
    fn from(value: UsbInterruptOutPipe) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbInterruptOutPipe> for ::windows::core::IInspectable {
    fn from(value: &UsbInterruptOutPipe) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbInterruptOutPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbInterruptOutPipe {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbInterruptOutPipe {}
unsafe impl ::core::marker::Sync for UsbInterruptOutPipe {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UsbReadOptions(pub u32);
impl UsbReadOptions {
    pub const None: Self = Self(0u32);
    pub const AutoClearStall: Self = Self(1u32);
    pub const OverrideAutomaticBufferManagement: Self = Self(2u32);
    pub const IgnoreShortPacket: Self = Self(4u32);
    pub const AllowPartialReads: Self = Self(8u32);
}
impl ::core::marker::Copy for UsbReadOptions {}
impl ::core::clone::Clone for UsbReadOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UsbReadOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for UsbReadOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbReadOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UsbReadOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UsbReadOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UsbReadOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UsbReadOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UsbReadOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for UsbReadOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Usb.UsbReadOptions;u4)");
}
impl ::windows::core::DefaultType for UsbReadOptions {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
pub struct UsbSetupPacket(::windows::core::IUnknown);
impl UsbSetupPacket {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UsbSetupPacket, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn RequestType(&self) -> ::windows::core::Result<UsbControlRequestType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UsbControlRequestType>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetRequestType<'a, Param0: ::windows::core::IntoParam<'a, UsbControlRequestType>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Request(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetRequest(&self, value: u8) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Value(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetValue(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Index(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'Devices_Usb'*"]
    pub fn SetLength(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'Devices_Usb', 'Storage_Streams'*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateWithEightByteBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(eightbytebuffer: Param0) -> ::windows::core::Result<UsbSetupPacket> {
        Self::IUsbSetupPacketFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), eightbytebuffer.into_param().abi(), &mut result__).from_abi::<UsbSetupPacket>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUsbSetupPacketFactory<R, F: FnOnce(&IUsbSetupPacketFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UsbSetupPacket, IUsbSetupPacketFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UsbSetupPacket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UsbSetupPacket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UsbSetupPacket {}
impl ::core::fmt::Debug for UsbSetupPacket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbSetupPacket").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbSetupPacket {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.Usb.UsbSetupPacket;{104ba132-c78f-4c51-b654-e49d02f2cb03})");
}
unsafe impl ::windows::core::Interface for UsbSetupPacket {
    type Vtable = IUsbSetupPacketVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x104ba132_c78f_4c51_b654_e49d02f2cb03);
}
impl ::windows::core::RuntimeName for UsbSetupPacket {
    const NAME: &'static str = "Windows.Devices.Usb.UsbSetupPacket";
}
impl ::core::convert::From<UsbSetupPacket> for ::windows::core::IUnknown {
    fn from(value: UsbSetupPacket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbSetupPacket> for ::windows::core::IUnknown {
    fn from(value: &UsbSetupPacket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UsbSetupPacket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &UsbSetupPacket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UsbSetupPacket> for ::windows::core::IInspectable {
    fn from(value: UsbSetupPacket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UsbSetupPacket> for ::windows::core::IInspectable {
    fn from(value: &UsbSetupPacket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UsbSetupPacket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &UsbSetupPacket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UsbSetupPacket {}
unsafe impl ::core::marker::Sync for UsbSetupPacket {}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UsbTransferDirection(pub i32);
impl UsbTransferDirection {
    pub const Out: Self = Self(0i32);
    pub const In: Self = Self(1i32);
}
impl ::core::marker::Copy for UsbTransferDirection {}
impl ::core::clone::Clone for UsbTransferDirection {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UsbTransferDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for UsbTransferDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbTransferDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UsbTransferDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Usb.UsbTransferDirection;i4)");
}
impl ::windows::core::DefaultType for UsbTransferDirection {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Devices_Usb'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UsbWriteOptions(pub u32);
impl UsbWriteOptions {
    pub const None: Self = Self(0u32);
    pub const AutoClearStall: Self = Self(1u32);
    pub const ShortPacketTerminate: Self = Self(2u32);
}
impl ::core::marker::Copy for UsbWriteOptions {}
impl ::core::clone::Clone for UsbWriteOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UsbWriteOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for UsbWriteOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UsbWriteOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for UsbWriteOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for UsbWriteOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for UsbWriteOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for UsbWriteOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for UsbWriteOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for UsbWriteOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.Usb.UsbWriteOptions;u4)");
}
impl ::windows::core::DefaultType for UsbWriteOptions {
    type DefaultType = Self;
}
