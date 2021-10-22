#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Devices_Printers_Extensions")]
pub mod Extensions;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IIppAttributeError(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIppAttributeError {
    type Vtable = IIppAttributeError_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1963978145,
        40687,
        23609,
        [147, 228, 70, 20, 155, 188, 239, 39],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeError_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut IppAttributeErrorReason,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::HRESULT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IIppAttributeValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIppAttributeValue {
    type Vtable = IIppAttributeValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2571141101,
        58043,
        22947,
        [152, 139, 40, 169, 116, 5, 42, 38],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeValue_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut IppAttributeValueKind,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IIppAttributeValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIppAttributeValueStatics {
    type Vtable = IIppAttributeValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        282343746,
        56724,
        22936,
        [178, 53, 175, 175, 182, 250, 121, 53],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppAttributeValueStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: i32,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Storage_Streams")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: super::super::Foundation::DateTime,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memberattributes: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        memberattributesarray: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        values: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IIppIntegerRange(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIppIntegerRange {
    type Vtable = IIppIntegerRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2458940230,
        50154,
        24278,
        [189, 177, 55, 82, 198, 44, 111, 127],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppIntegerRange_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IIppIntegerRangeFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIppIntegerRangeFactory {
    type Vtable = IIppIntegerRangeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1976888494,
        63614,
        21677,
        [181, 208, 70, 82, 4, 219, 117, 83],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppIntegerRangeFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        start: i32,
        end: i32,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IIppPrintDevice(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIppPrintDevice {
    type Vtable = IIppPrintDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3611864150,
        30451,
        24006,
        [175, 212, 194, 168, 104, 107, 147, 89],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppPrintDevice_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        attributenames: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        attributenames: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Storage_Streams")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        printerattributesbuffer: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        printerattributes: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IIppResolution(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIppResolution {
    type Vtable = IIppResolution_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3410575238,
        27635,
        22261,
        [134, 206, 38, 61, 8, 174, 173, 99],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppResolution_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut IppResolutionUnit,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IIppResolutionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIppResolutionFactory {
    type Vtable = IIppResolutionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3833709230,
        9498,
        21286,
        [177, 115, 149, 84, 62, 217, 154, 53],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppResolutionFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: i32,
        height: i32,
        unit: IppResolutionUnit,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IIppSetAttributesResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIppSetAttributesResult {
    type Vtable = IIppSetAttributesResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2099019605,
        43677,
        22691,
        [144, 233, 23, 189, 197, 40, 31, 7],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppSetAttributesResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IIppTextWithLanguage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIppTextWithLanguage {
    type Vtable = IIppTextWithLanguage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        845432742,
        20809,
        22838,
        [144, 232, 12, 115, 96, 54, 191, 119],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppTextWithLanguage_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IIppTextWithLanguageFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIppTextWithLanguageFactory {
    type Vtable = IIppTextWithLanguageFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3393855117,
        10600,
        22389,
        [153, 124, 138, 70, 241, 165, 116, 237],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IIppTextWithLanguageFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        language: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IPrint3DDevice(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrint3DDevice {
    type Vtable = IPrint3DDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        68959513,
        38675,
        17058,
        [152, 19, 125, 195, 51, 116, 40, 211],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DDevice_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IPrint3DDeviceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrint3DDeviceStatics {
    type Vtable = IPrint3DDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4259537418,
        26573,
        16823,
        [163, 68, 81, 80, 161, 253, 117, 181],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DDeviceStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IPrintSchema(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintSchema {
    type Vtable = IPrintSchema_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3266937622,
        9912,
        19451,
        [129, 56, 159, 150, 44, 34, 163, 91],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSchema_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        constrainticket: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deltaticket: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IppAttributeError(::windows::runtime::IInspectable);
impl IppAttributeError {
    pub fn Reason(&self) -> ::windows::runtime::Result<IppAttributeErrorReason> {
        let this = self;
        unsafe {
            let mut result__: IppAttributeErrorReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IppAttributeErrorReason>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUnsupportedValues(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<IppAttributeValue>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<IppAttributeValue>>(
                result__,
            )
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IppAttributeError {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Devices.Printers.IppAttributeError;{750feda1-9eef-5c39-93e4-46149bbcef27})",
    );
}
unsafe impl ::windows::runtime::Interface for IppAttributeError {
    type Vtable = IIppAttributeError_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1963978145,
        40687,
        23609,
        [147, 228, 70, 20, 155, 188, 239, 39],
    );
}
impl ::windows::runtime::RuntimeName for IppAttributeError {
    const NAME: &'static str = "Windows.Devices.Printers.IppAttributeError";
}
impl ::std::convert::From<IppAttributeError> for ::windows::runtime::IUnknown {
    fn from(value: IppAttributeError) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IppAttributeError> for ::windows::runtime::IUnknown {
    fn from(value: &IppAttributeError) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IppAttributeError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IppAttributeError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IppAttributeError> for ::windows::runtime::IInspectable {
    fn from(value: IppAttributeError) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IppAttributeError> for ::windows::runtime::IInspectable {
    fn from(value: &IppAttributeError) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IppAttributeError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IppAttributeError
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for IppAttributeError {}
unsafe impl ::std::marker::Sync for IppAttributeError {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IppAttributeErrorReason(pub i32);
impl IppAttributeErrorReason {
    pub const RequestEntityTooLarge: IppAttributeErrorReason = IppAttributeErrorReason(0i32);
    pub const AttributeNotSupported: IppAttributeErrorReason = IppAttributeErrorReason(1i32);
    pub const AttributeValuesNotSupported: IppAttributeErrorReason = IppAttributeErrorReason(2i32);
    pub const AttributeNotSettable: IppAttributeErrorReason = IppAttributeErrorReason(3i32);
    pub const ConflictingAttributes: IppAttributeErrorReason = IppAttributeErrorReason(4i32);
}
impl ::std::convert::From<i32> for IppAttributeErrorReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IppAttributeErrorReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IppAttributeErrorReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Devices.Printers.IppAttributeErrorReason;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IppAttributeValue(::windows::runtime::IInspectable);
impl IppAttributeValue {
    pub fn Kind(&self) -> ::windows::runtime::Result<IppAttributeValueKind> {
        let this = self;
        unsafe {
            let mut result__: IppAttributeValueKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IppAttributeValueKind>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntegerArray(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBooleanArray(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEnumArray(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<i32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetOctetStringArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::super::Storage::Streams::IBuffer,
            >>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetDateTimeArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::super::Foundation::DateTime,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetResolutionArray(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<IppResolution>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<IppResolution>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRangeOfIntegerArray(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<IppIntegerRange>>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<IppIntegerRange>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCollectionArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<
            super::super::Foundation::Collections::IMapView<
                ::windows::runtime::HSTRING,
                IppAttributeValue,
            >,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::super::Foundation::Collections::IMapView<
                    ::windows::runtime::HSTRING,
                    IppAttributeValue,
                >,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextWithLanguageArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<IppTextWithLanguage>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<IppTextWithLanguage>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNameWithLanguageArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<IppTextWithLanguage>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<IppTextWithLanguage>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextWithoutLanguageArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .17 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Foundation::Collections:: IVector :: < :: windows :: runtime :: HSTRING > > ( result__ )
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNameWithoutLanguageArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .18 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Foundation::Collections:: IVector :: < :: windows :: runtime :: HSTRING > > ( result__ )
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetKeywordArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .19 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Foundation::Collections:: IVector :: < :: windows :: runtime :: HSTRING > > ( result__ )
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetUriArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<
                super::super::Foundation::Uri,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetUriSchemaArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .21 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Foundation::Collections:: IVector :: < :: windows :: runtime :: HSTRING > > ( result__ )
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCharsetArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .22 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Foundation::Collections:: IVector :: < :: windows :: runtime :: HSTRING > > ( result__ )
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNaturalLanguageArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .23 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Foundation::Collections:: IVector :: < :: windows :: runtime :: HSTRING > > ( result__ )
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMimeMediaTypeArray(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .24 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Foundation::Collections:: IVector :: < :: windows :: runtime :: HSTRING > > ( result__ )
        }
    }
    pub fn CreateUnsupported() -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateUnknown() -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateNoValue() -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateInteger(value: i32) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateIntegerArray<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i32>>,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateBoolean(value: bool) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateBooleanArray<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<bool>>,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateEnum(value: i32) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateEnumArray<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<i32>>,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateOctetString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>,
    >(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn CreateOctetStringArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<
                super::super::Storage::Streams::IBuffer,
            >,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateDateTime<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>,
    >(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn CreateDateTimeArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<super::super::Foundation::DateTime>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateResolution<'a, Param0: ::windows::runtime::IntoParam<'a, IppResolution>>(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateResolutionArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<IppResolution>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateRangeOfInteger<'a, Param0: ::windows::runtime::IntoParam<'a, IppIntegerRange>>(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateRangeOfIntegerArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<IppIntegerRange>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCollection<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<
                super::super::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    IppAttributeValue,
                >,
            >,
        >,
    >(
        memberattributes: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                memberattributes.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCollectionArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<
                super::super::Foundation::Collections::IIterable<
                    super::super::Foundation::Collections::IKeyValuePair<
                        ::windows::runtime::HSTRING,
                        IppAttributeValue,
                    >,
                >,
            >,
        >,
    >(
        memberattributesarray: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                memberattributesarray.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateTextWithLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IppTextWithLanguage>,
    >(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateTextWithLanguageArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<IppTextWithLanguage>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateNameWithLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IppTextWithLanguage>,
    >(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNameWithLanguageArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<IppTextWithLanguage>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateTextWithoutLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateTextWithoutLanguageArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateNameWithoutLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNameWithoutLanguageArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateKeyword<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateKeywordArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateUri<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
    >(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn CreateUriArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateUriSchema<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateUriSchemaArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateCharset<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCharsetArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateNaturalLanguage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateNaturalLanguageArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn CreateMimeMedia<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        value: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateMimeMediaArray<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        values: Param0,
    ) -> ::windows::runtime::Result<IppAttributeValue> {
        Self::IIppAttributeValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                values.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppAttributeValue>(result__)
        })
    }
    pub fn IIppAttributeValueStatics<
        R,
        F: FnOnce(&IIppAttributeValueStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            IppAttributeValue,
            IIppAttributeValueStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IppAttributeValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Devices.Printers.IppAttributeValue;{99407fed-e2bb-59a3-988b-28a974052a26})",
    );
}
unsafe impl ::windows::runtime::Interface for IppAttributeValue {
    type Vtable = IIppAttributeValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2571141101,
        58043,
        22947,
        [152, 139, 40, 169, 116, 5, 42, 38],
    );
}
impl ::windows::runtime::RuntimeName for IppAttributeValue {
    const NAME: &'static str = "Windows.Devices.Printers.IppAttributeValue";
}
impl ::std::convert::From<IppAttributeValue> for ::windows::runtime::IUnknown {
    fn from(value: IppAttributeValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IppAttributeValue> for ::windows::runtime::IUnknown {
    fn from(value: &IppAttributeValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IppAttributeValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IppAttributeValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IppAttributeValue> for ::windows::runtime::IInspectable {
    fn from(value: IppAttributeValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IppAttributeValue> for ::windows::runtime::IInspectable {
    fn from(value: &IppAttributeValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IppAttributeValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IppAttributeValue
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for IppAttributeValue {}
unsafe impl ::std::marker::Sync for IppAttributeValue {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IppAttributeValueKind(pub i32);
impl IppAttributeValueKind {
    pub const Unsupported: IppAttributeValueKind = IppAttributeValueKind(0i32);
    pub const Unknown: IppAttributeValueKind = IppAttributeValueKind(1i32);
    pub const NoValue: IppAttributeValueKind = IppAttributeValueKind(2i32);
    pub const Integer: IppAttributeValueKind = IppAttributeValueKind(3i32);
    pub const Boolean: IppAttributeValueKind = IppAttributeValueKind(4i32);
    pub const Enum: IppAttributeValueKind = IppAttributeValueKind(5i32);
    pub const OctetString: IppAttributeValueKind = IppAttributeValueKind(6i32);
    pub const DateTime: IppAttributeValueKind = IppAttributeValueKind(7i32);
    pub const Resolution: IppAttributeValueKind = IppAttributeValueKind(8i32);
    pub const RangeOfInteger: IppAttributeValueKind = IppAttributeValueKind(9i32);
    pub const Collection: IppAttributeValueKind = IppAttributeValueKind(10i32);
    pub const TextWithLanguage: IppAttributeValueKind = IppAttributeValueKind(11i32);
    pub const NameWithLanguage: IppAttributeValueKind = IppAttributeValueKind(12i32);
    pub const TextWithoutLanguage: IppAttributeValueKind = IppAttributeValueKind(13i32);
    pub const NameWithoutLanguage: IppAttributeValueKind = IppAttributeValueKind(14i32);
    pub const Keyword: IppAttributeValueKind = IppAttributeValueKind(15i32);
    pub const Uri: IppAttributeValueKind = IppAttributeValueKind(16i32);
    pub const UriSchema: IppAttributeValueKind = IppAttributeValueKind(17i32);
    pub const Charset: IppAttributeValueKind = IppAttributeValueKind(18i32);
    pub const NaturalLanguage: IppAttributeValueKind = IppAttributeValueKind(19i32);
    pub const MimeMediaType: IppAttributeValueKind = IppAttributeValueKind(20i32);
}
impl ::std::convert::From<i32> for IppAttributeValueKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IppAttributeValueKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IppAttributeValueKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Devices.Printers.IppAttributeValueKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IppIntegerRange(::windows::runtime::IInspectable);
impl IppIntegerRange {
    pub fn Start(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn End(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn CreateInstance(start: i32, end: i32) -> ::windows::runtime::Result<IppIntegerRange> {
        Self::IIppIntegerRangeFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                start,
                end,
                &mut result__,
            )
            .from_abi::<IppIntegerRange>(result__)
        })
    }
    pub fn IIppIntegerRangeFactory<
        R,
        F: FnOnce(&IIppIntegerRangeFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            IppIntegerRange,
            IIppIntegerRangeFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IppIntegerRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Devices.Printers.IppIntegerRange;{92907346-c3ea-5ed6-bdb1-3752c62c6f7f})",
    );
}
unsafe impl ::windows::runtime::Interface for IppIntegerRange {
    type Vtable = IIppIntegerRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2458940230,
        50154,
        24278,
        [189, 177, 55, 82, 198, 44, 111, 127],
    );
}
impl ::windows::runtime::RuntimeName for IppIntegerRange {
    const NAME: &'static str = "Windows.Devices.Printers.IppIntegerRange";
}
impl ::std::convert::From<IppIntegerRange> for ::windows::runtime::IUnknown {
    fn from(value: IppIntegerRange) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IppIntegerRange> for ::windows::runtime::IUnknown {
    fn from(value: &IppIntegerRange) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IppIntegerRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IppIntegerRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IppIntegerRange> for ::windows::runtime::IInspectable {
    fn from(value: IppIntegerRange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IppIntegerRange> for ::windows::runtime::IInspectable {
    fn from(value: &IppIntegerRange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IppIntegerRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IppIntegerRange
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for IppIntegerRange {}
unsafe impl ::std::marker::Sync for IppIntegerRange {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IppPrintDevice(::windows::runtime::IInspectable);
impl IppPrintDevice {
    pub fn PrinterName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PrinterUri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn GetPrinterAttributesAsBuffer<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        &self,
        attributenames: Param0,
    ) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                attributenames.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPrinterAttributes<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        &self,
        attributenames: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, IppAttributeValue>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                attributenames.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                IppAttributeValue,
            >>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPrinterAttributesFromBuffer<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>,
    >(
        &self,
        printerattributesbuffer: Param0,
    ) -> ::windows::runtime::Result<IppSetAttributesResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                printerattributesbuffer.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppSetAttributesResult>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPrinterAttributes<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<
                super::super::Foundation::Collections::IKeyValuePair<
                    ::windows::runtime::HSTRING,
                    IppAttributeValue,
                >,
            >,
        >,
    >(
        &self,
        printerattributes: Param0,
    ) -> ::windows::runtime::Result<IppSetAttributesResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                printerattributes.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppSetAttributesResult>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IppPrintDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Devices.Printers.IppPrintDevice;{d748ac56-76f3-5dc6-afd4-c2a8686b9359})",
    );
}
unsafe impl ::windows::runtime::Interface for IppPrintDevice {
    type Vtable = IIppPrintDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3611864150,
        30451,
        24006,
        [175, 212, 194, 168, 104, 107, 147, 89],
    );
}
impl ::windows::runtime::RuntimeName for IppPrintDevice {
    const NAME: &'static str = "Windows.Devices.Printers.IppPrintDevice";
}
impl ::std::convert::From<IppPrintDevice> for ::windows::runtime::IUnknown {
    fn from(value: IppPrintDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IppPrintDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IppPrintDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IppPrintDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IppPrintDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IppPrintDevice> for ::windows::runtime::IInspectable {
    fn from(value: IppPrintDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IppPrintDevice> for ::windows::runtime::IInspectable {
    fn from(value: &IppPrintDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IppPrintDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IppPrintDevice
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for IppPrintDevice {}
unsafe impl ::std::marker::Sync for IppPrintDevice {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IppResolution(::windows::runtime::IInspectable);
impl IppResolution {
    pub fn Width(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn Height(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn Unit(&self) -> ::windows::runtime::Result<IppResolutionUnit> {
        let this = self;
        unsafe {
            let mut result__: IppResolutionUnit = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IppResolutionUnit>(result__)
        }
    }
    pub fn CreateInstance(
        width: i32,
        height: i32,
        unit: IppResolutionUnit,
    ) -> ::windows::runtime::Result<IppResolution> {
        Self::IIppResolutionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                width,
                height,
                unit,
                &mut result__,
            )
            .from_abi::<IppResolution>(result__)
        })
    }
    pub fn IIppResolutionFactory<
        R,
        F: FnOnce(&IIppResolutionFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IppResolution, IIppResolutionFactory> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IppResolution {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Devices.Printers.IppResolution;{cb493f86-6bf3-56f5-86ce-263d08aead63})",
    );
}
unsafe impl ::windows::runtime::Interface for IppResolution {
    type Vtable = IIppResolution_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3410575238,
        27635,
        22261,
        [134, 206, 38, 61, 8, 174, 173, 99],
    );
}
impl ::windows::runtime::RuntimeName for IppResolution {
    const NAME: &'static str = "Windows.Devices.Printers.IppResolution";
}
impl ::std::convert::From<IppResolution> for ::windows::runtime::IUnknown {
    fn from(value: IppResolution) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IppResolution> for ::windows::runtime::IUnknown {
    fn from(value: &IppResolution) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IppResolution {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IppResolution {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IppResolution> for ::windows::runtime::IInspectable {
    fn from(value: IppResolution) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IppResolution> for ::windows::runtime::IInspectable {
    fn from(value: &IppResolution) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IppResolution {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IppResolution {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for IppResolution {}
unsafe impl ::std::marker::Sync for IppResolution {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct IppResolutionUnit(pub i32);
impl IppResolutionUnit {
    pub const DotsPerInch: IppResolutionUnit = IppResolutionUnit(0i32);
    pub const DotsPerCentimeter: IppResolutionUnit = IppResolutionUnit(1i32);
}
impl ::std::convert::From<i32> for IppResolutionUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IppResolutionUnit {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IppResolutionUnit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Devices.Printers.IppResolutionUnit;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IppSetAttributesResult(::windows::runtime::IInspectable);
impl IppSetAttributesResult {
    pub fn Succeeded(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AttributeErrors(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMapView<
            ::windows::runtime::HSTRING,
            IppAttributeError,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMapView<
                ::windows::runtime::HSTRING,
                IppAttributeError,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IppSetAttributesResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Devices.Printers.IppSetAttributesResult;{7d1c7f55-aa9d-58a3-90e9-17bdc5281f07})" ) ;
}
unsafe impl ::windows::runtime::Interface for IppSetAttributesResult {
    type Vtable = IIppSetAttributesResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2099019605,
        43677,
        22691,
        [144, 233, 23, 189, 197, 40, 31, 7],
    );
}
impl ::windows::runtime::RuntimeName for IppSetAttributesResult {
    const NAME: &'static str = "Windows.Devices.Printers.IppSetAttributesResult";
}
impl ::std::convert::From<IppSetAttributesResult> for ::windows::runtime::IUnknown {
    fn from(value: IppSetAttributesResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IppSetAttributesResult> for ::windows::runtime::IUnknown {
    fn from(value: &IppSetAttributesResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IppSetAttributesResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IppSetAttributesResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IppSetAttributesResult> for ::windows::runtime::IInspectable {
    fn from(value: IppSetAttributesResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IppSetAttributesResult> for ::windows::runtime::IInspectable {
    fn from(value: &IppSetAttributesResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IppSetAttributesResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IppSetAttributesResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for IppSetAttributesResult {}
unsafe impl ::std::marker::Sync for IppSetAttributesResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IppTextWithLanguage(::windows::runtime::IInspectable);
impl IppTextWithLanguage {
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn CreateInstance<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        language: Param0,
        text: Param1,
    ) -> ::windows::runtime::Result<IppTextWithLanguage> {
        Self::IIppTextWithLanguageFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                language.into_param().abi(),
                text.into_param().abi(),
                &mut result__,
            )
            .from_abi::<IppTextWithLanguage>(result__)
        })
    }
    pub fn IIppTextWithLanguageFactory<
        R,
        F: FnOnce(&IIppTextWithLanguageFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            IppTextWithLanguage,
            IIppTextWithLanguageFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IppTextWithLanguage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Devices.Printers.IppTextWithLanguage;{326447a6-5149-5936-90e8-0c736036bf77})",
    );
}
unsafe impl ::windows::runtime::Interface for IppTextWithLanguage {
    type Vtable = IIppTextWithLanguage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        845432742,
        20809,
        22838,
        [144, 232, 12, 115, 96, 54, 191, 119],
    );
}
impl ::windows::runtime::RuntimeName for IppTextWithLanguage {
    const NAME: &'static str = "Windows.Devices.Printers.IppTextWithLanguage";
}
impl ::std::convert::From<IppTextWithLanguage> for ::windows::runtime::IUnknown {
    fn from(value: IppTextWithLanguage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IppTextWithLanguage> for ::windows::runtime::IUnknown {
    fn from(value: &IppTextWithLanguage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IppTextWithLanguage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IppTextWithLanguage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IppTextWithLanguage> for ::windows::runtime::IInspectable {
    fn from(value: IppTextWithLanguage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IppTextWithLanguage> for ::windows::runtime::IInspectable {
    fn from(value: &IppTextWithLanguage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IppTextWithLanguage
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IppTextWithLanguage
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for IppTextWithLanguage {}
unsafe impl ::std::marker::Sync for IppTextWithLanguage {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct Print3DDevice(::windows::runtime::IInspectable);
impl Print3DDevice {
    pub fn PrintSchema(&self) -> ::windows::runtime::Result<PrintSchema> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PrintSchema>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        deviceid: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<Print3DDevice>> {
        Self::IPrint3DDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                deviceid.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<Print3DDevice>>(result__)
        })
    }
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IPrint3DDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IPrint3DDeviceStatics<
        R,
        F: FnOnce(&IPrint3DDeviceStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Print3DDevice, IPrint3DDeviceStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Print3DDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Devices.Printers.Print3DDevice;{041c3d19-9713-42a2-9813-7dc3337428d3})",
    );
}
unsafe impl ::windows::runtime::Interface for Print3DDevice {
    type Vtable = IPrint3DDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        68959513,
        38675,
        17058,
        [152, 19, 125, 195, 51, 116, 40, 211],
    );
}
impl ::windows::runtime::RuntimeName for Print3DDevice {
    const NAME: &'static str = "Windows.Devices.Printers.Print3DDevice";
}
impl ::std::convert::From<Print3DDevice> for ::windows::runtime::IUnknown {
    fn from(value: Print3DDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&Print3DDevice> for ::windows::runtime::IUnknown {
    fn from(value: &Print3DDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for Print3DDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &Print3DDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<Print3DDevice> for ::windows::runtime::IInspectable {
    fn from(value: Print3DDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Print3DDevice> for ::windows::runtime::IInspectable {
    fn from(value: &Print3DDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for Print3DDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a Print3DDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for Print3DDevice {}
unsafe impl ::std::marker::Sync for Print3DDevice {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct PrintSchema(::windows::runtime::IInspectable);
impl PrintSchema {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetDefaultPrintTicketAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<
            super::super::Storage::Streams::IRandomAccessStreamWithContentType,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                super::super::Storage::Streams::IRandomAccessStreamWithContentType,
            >>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetCapabilitiesAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Storage::Streams::IRandomAccessStreamWithContentType,
        >,
    >(
        &self,
        constrainticket: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<
            super::super::Storage::Streams::IRandomAccessStreamWithContentType,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                constrainticket.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                super::super::Storage::Streams::IRandomAccessStreamWithContentType,
            >>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn MergeAndValidateWithDefaultPrintTicketAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Storage::Streams::IRandomAccessStreamWithContentType,
        >,
    >(
        &self,
        deltaticket: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<
            super::super::Storage::Streams::IRandomAccessStreamWithContentType,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                deltaticket.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                super::super::Storage::Streams::IRandomAccessStreamWithContentType,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintSchema {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Devices.Printers.PrintSchema;{c2b98316-26b8-4bfb-8138-9f962c22a35b})",
    );
}
unsafe impl ::windows::runtime::Interface for PrintSchema {
    type Vtable = IPrintSchema_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3266937622,
        9912,
        19451,
        [129, 56, 159, 150, 44, 34, 163, 91],
    );
}
impl ::windows::runtime::RuntimeName for PrintSchema {
    const NAME: &'static str = "Windows.Devices.Printers.PrintSchema";
}
impl ::std::convert::From<PrintSchema> for ::windows::runtime::IUnknown {
    fn from(value: PrintSchema) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PrintSchema> for ::windows::runtime::IUnknown {
    fn from(value: &PrintSchema) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PrintSchema {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PrintSchema {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<PrintSchema> for ::windows::runtime::IInspectable {
    fn from(value: PrintSchema) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PrintSchema> for ::windows::runtime::IInspectable {
    fn from(value: &PrintSchema) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PrintSchema {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PrintSchema {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PrintSchema {}
unsafe impl ::std::marker::Sync for PrintSchema {}
#[repr(C)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
)]
pub struct PrintersContract(pub u8);
