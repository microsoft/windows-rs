#[doc(hidden)]
#[repr(transparent)]
pub struct IHidBooleanControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHidBooleanControl {
    type Vtable = IHidBooleanControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IHidBooleanControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x524df48a_3695_408c_bba2_e2eb5abfbc20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ControlDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidBooleanControlDescription(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHidBooleanControlDescription {
    type Vtable = IHidBooleanControlDescription_Vtbl;
}
unsafe impl ::windows::core::Interface for IHidBooleanControlDescription {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6196e543_29d8_4a2a_8683_849e207bbe31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ReportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ReportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HidReportType) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ParentCollections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ParentCollections: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidBooleanControlDescription2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHidBooleanControlDescription2 {
    type Vtable = IHidBooleanControlDescription2_Vtbl;
}
unsafe impl ::windows::core::Interface for IHidBooleanControlDescription2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8eed2ea_8a77_4c36_aa00_5ff0449d3e73);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsAbsolute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidCollection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHidCollection {
    type Vtable = IHidCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IHidCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7189f5a3_32f1_46e3_befd_44d2663b7e6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidCollection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HidCollectionType) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHidDevice {
    type Vtable = IHidDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for IHidDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f8a14e7_2200_432e_95da_d09b87d574a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub VendorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ProductId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetInputReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetInputReportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetInputReportByIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetInputReportByIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFeatureReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFeatureReportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFeatureReportByIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFeatureReportByIdAsync: usize,
    pub CreateOutputReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateOutputReportById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFeatureReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFeatureReportById: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reportid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SendOutputReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputreport: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendOutputReportAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SendFeatureReportAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, featurereport: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SendFeatureReportAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBooleanControlDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBooleanControlDescriptions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNumericControlDescriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNumericControlDescriptions: usize,
    #[cfg(feature = "Foundation")]
    pub InputReportReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reporthandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InputReportReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInputReportReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInputReportReceived: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHidDeviceStatics {
    type Vtable = IHidDeviceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IHidDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e5981e4_9856_418c_9f73_77de0cd85754);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorVidPid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, vendorid: u16, productid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: *mut ::core::ffi::c_void, accessmode: super::super::Storage::FileAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    FromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidFeatureReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHidFeatureReport {
    type Vtable = IHidFeatureReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IHidFeatureReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x841d9b79_5ae5_46e3_82ef_1fec5c8942f4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidFeatureReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    pub GetBooleanControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidInputReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHidInputReport {
    type Vtable = IHidInputReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IHidInputReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc35d0e50_f7e7_4e8d_b23e_cabbe56b90e9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidInputReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ActivatedBooleanControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActivatedBooleanControls: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub TransitionedBooleanControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TransitionedBooleanControls: usize,
    pub GetBooleanControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidInputReportReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHidInputReportReceivedEventArgs {
    type Vtable = IHidInputReportReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IHidInputReportReceivedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7059c5cb_59b2_4dc2_985c_0adc6136fa2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidInputReportReceivedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidNumericControl(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHidNumericControl {
    type Vtable = IHidNumericControl_Vtbl;
}
unsafe impl ::windows::core::Interface for IHidNumericControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe38a12a5_35a7_4b75_89c8_fb1f28b10823);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidNumericControl_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsGrouped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT,
    pub ScaledValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub SetScaledValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64) -> ::windows::core::HRESULT,
    pub ControlDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidNumericControlDescription(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHidNumericControlDescription {
    type Vtable = IHidNumericControlDescription_Vtbl;
}
unsafe impl ::windows::core::Interface for IHidNumericControlDescription {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x638d5e86_1d97_4c75_927f_5ff58ba05e32);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidNumericControlDescription_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ReportId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub ReportType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HidReportType) -> ::windows::core::HRESULT,
    pub ReportSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ReportCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsagePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub UsageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub LogicalMinimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub LogicalMaximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub PhysicalMinimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub PhysicalMaximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub UnitExponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsAbsolute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasNull: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ParentCollections: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ParentCollections: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHidOutputReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IHidOutputReport {
    type Vtable = IHidOutputReport_Vtbl;
}
unsafe impl ::windows::core::Interface for IHidOutputReport {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x62cb2544_c896_4463_93c1_df9db053c450);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidOutputReport_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    pub GetBooleanControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBooleanControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNumericControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usagepage: u16, usageid: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNumericControlByDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controldescription: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidBooleanControl(::windows::core::IUnknown);
impl HidBooleanControl {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsagePage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsageId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsActive)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsActive(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsActive)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ControlDescription(&self) -> ::windows::core::Result<HidBooleanControlDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ControlDescription)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HidBooleanControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidBooleanControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidBooleanControl {}
impl ::core::fmt::Debug for HidBooleanControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidBooleanControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidBooleanControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidBooleanControl;{524df48a-3695-408c-bba2-e2eb5abfbc20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HidBooleanControl {
    type Vtable = IHidBooleanControl_Vtbl;
}
unsafe impl ::windows::core::Interface for HidBooleanControl {
    const IID: ::windows::core::GUID = <IHidBooleanControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidBooleanControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidBooleanControl";
}
::windows::core::interface_hierarchy!(HidBooleanControl, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HidBooleanControl {}
unsafe impl ::core::marker::Sync for HidBooleanControl {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidBooleanControlDescription(::windows::core::IUnknown);
impl HidBooleanControlDescription {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReportId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReportType(&self) -> ::windows::core::Result<HidReportType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsagePage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsageId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ParentCollections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentCollections)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsAbsolute(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IHidBooleanControlDescription2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAbsolute)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HidBooleanControlDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidBooleanControlDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidBooleanControlDescription {}
impl ::core::fmt::Debug for HidBooleanControlDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidBooleanControlDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidBooleanControlDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription;{6196e543-29d8-4a2a-8683-849e207bbe31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HidBooleanControlDescription {
    type Vtable = IHidBooleanControlDescription_Vtbl;
}
unsafe impl ::windows::core::Interface for HidBooleanControlDescription {
    const IID: ::windows::core::GUID = <IHidBooleanControlDescription as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidBooleanControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription";
}
::windows::core::interface_hierarchy!(HidBooleanControlDescription, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HidBooleanControlDescription {}
unsafe impl ::core::marker::Sync for HidBooleanControlDescription {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidCollection(::windows::core::IUnknown);
impl HidCollection {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<HidCollectionType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsagePage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsageId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HidCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidCollection {}
impl ::core::fmt::Debug for HidCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidCollection;{7189f5a3-32f1-46e3-befd-44d2663b7e6a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HidCollection {
    type Vtable = IHidCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for HidCollection {
    const IID: ::windows::core::GUID = <IHidCollection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidCollection {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidCollection";
}
::windows::core::interface_hierarchy!(HidCollection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HidCollection {}
unsafe impl ::core::marker::Sync for HidCollection {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidDevice(::windows::core::IUnknown);
impl HidDevice {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn VendorId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).VendorId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ProductId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ProductId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Version(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Version)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsagePage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsageId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetInputReportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInputReportAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetInputReportByIdAsync(&self, reportid: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidInputReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInputReportByIdAsync)(::windows::core::Vtable::as_raw(this), reportid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFeatureReportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFeatureReportAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFeatureReportByIdAsync(&self, reportid: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFeatureReportByIdAsync)(::windows::core::Vtable::as_raw(this), reportid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateOutputReport(&self) -> ::windows::core::Result<HidOutputReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateOutputReport)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateOutputReportById(&self, reportid: u16) -> ::windows::core::Result<HidOutputReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateOutputReportById)(::windows::core::Vtable::as_raw(this), reportid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateFeatureReport(&self) -> ::windows::core::Result<HidFeatureReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFeatureReport)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateFeatureReportById(&self, reportid: u16) -> ::windows::core::Result<HidFeatureReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateFeatureReportById)(::windows::core::Vtable::as_raw(this), reportid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendOutputReportAsync(&self, outputreport: &HidOutputReport) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendOutputReportAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(outputreport), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SendFeatureReportAsync(&self, featurereport: &HidFeatureReport) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SendFeatureReportAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(featurereport), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBooleanControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBooleanControlDescriptions)(::windows::core::Vtable::as_raw(this), reporttype, usagepage, usageid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNumericControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumericControlDescriptions)(::windows::core::Vtable::as_raw(this), reporttype, usagepage, usageid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InputReportReceived(&self, reporthandler: &super::super::Foundation::TypedEventHandler<HidDevice, HidInputReportReceivedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InputReportReceived)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(reporthandler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputReportReceived(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveInputReportReceived)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn GetDeviceSelector(usagepage: u16, usageid: u16) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelector)(::windows::core::Vtable::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GetDeviceSelectorVidPid(usagepage: u16, usageid: u16, vendorid: u16, productid: u16) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeviceSelectorVidPid)(::windows::core::Vtable::as_raw(this), usagepage, usageid, vendorid, productid, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn FromIdAsync(deviceid: &::windows::core::HSTRING, accessmode: super::super::Storage::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HidDevice>> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FromIdAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(deviceid), accessmode, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IHidDeviceStatics<R, F: FnOnce(&IHidDeviceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<HidDevice, IHidDeviceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for HidDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidDevice {}
impl ::core::fmt::Debug for HidDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidDevice;{5f8a14e7-2200-432e-95da-d09b87d574a8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HidDevice {
    type Vtable = IHidDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for HidDevice {
    const IID: ::windows::core::GUID = <IHidDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidDevice {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidDevice";
}
::windows::core::interface_hierarchy!(HidDevice, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HidDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HidDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HidDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HidDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HidDevice> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HidDevice) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HidDevice {}
unsafe impl ::core::marker::Sync for HidDevice {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidFeatureReport(::windows::core::IUnknown);
impl HidFeatureReport {
    pub fn Id(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBooleanControl)(::windows::core::Vtable::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBooleanControlByDescription(&self, controldescription: &HidBooleanControlDescription) -> ::windows::core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBooleanControlByDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(controldescription), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumericControl)(::windows::core::Vtable::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNumericControlByDescription(&self, controldescription: &HidNumericControlDescription) -> ::windows::core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumericControlByDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(controldescription), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HidFeatureReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidFeatureReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidFeatureReport {}
impl ::core::fmt::Debug for HidFeatureReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidFeatureReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidFeatureReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidFeatureReport;{841d9b79-5ae5-46e3-82ef-1fec5c8942f4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HidFeatureReport {
    type Vtable = IHidFeatureReport_Vtbl;
}
unsafe impl ::windows::core::Interface for HidFeatureReport {
    const IID: ::windows::core::GUID = <IHidFeatureReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidFeatureReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidFeatureReport";
}
::windows::core::interface_hierarchy!(HidFeatureReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HidFeatureReport {}
unsafe impl ::core::marker::Sync for HidFeatureReport {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidInputReport(::windows::core::IUnknown);
impl HidInputReport {
    pub fn Id(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActivatedBooleanControls(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ActivatedBooleanControls)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TransitionedBooleanControls(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TransitionedBooleanControls)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBooleanControl)(::windows::core::Vtable::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBooleanControlByDescription(&self, controldescription: &HidBooleanControlDescription) -> ::windows::core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBooleanControlByDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(controldescription), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumericControl)(::windows::core::Vtable::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNumericControlByDescription(&self, controldescription: &HidNumericControlDescription) -> ::windows::core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumericControlByDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(controldescription), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HidInputReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidInputReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidInputReport {}
impl ::core::fmt::Debug for HidInputReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidInputReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidInputReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidInputReport;{c35d0e50-f7e7-4e8d-b23e-cabbe56b90e9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HidInputReport {
    type Vtable = IHidInputReport_Vtbl;
}
unsafe impl ::windows::core::Interface for HidInputReport {
    const IID: ::windows::core::GUID = <IHidInputReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidInputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidInputReport";
}
::windows::core::interface_hierarchy!(HidInputReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HidInputReport {}
unsafe impl ::core::marker::Sync for HidInputReport {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidInputReportReceivedEventArgs(::windows::core::IUnknown);
impl HidInputReportReceivedEventArgs {
    pub fn Report(&self) -> ::windows::core::Result<HidInputReport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Report)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HidInputReportReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidInputReportReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidInputReportReceivedEventArgs {}
impl ::core::fmt::Debug for HidInputReportReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidInputReportReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidInputReportReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs;{7059c5cb-59b2-4dc2-985c-0adc6136fa2d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HidInputReportReceivedEventArgs {
    type Vtable = IHidInputReportReceivedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for HidInputReportReceivedEventArgs {
    const IID: ::windows::core::GUID = <IHidInputReportReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidInputReportReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs";
}
::windows::core::interface_hierarchy!(HidInputReportReceivedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HidInputReportReceivedEventArgs {}
unsafe impl ::core::marker::Sync for HidInputReportReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidNumericControl(::windows::core::IUnknown);
impl HidNumericControl {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsGrouped(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsGrouped)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsagePage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsageId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetValue)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ScaledValue(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScaledValue)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetScaledValue(&self, value: i64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScaledValue)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ControlDescription(&self) -> ::windows::core::Result<HidNumericControlDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ControlDescription)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HidNumericControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidNumericControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidNumericControl {}
impl ::core::fmt::Debug for HidNumericControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidNumericControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidNumericControl {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidNumericControl;{e38a12a5-35a7-4b75-89c8-fb1f28b10823})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HidNumericControl {
    type Vtable = IHidNumericControl_Vtbl;
}
unsafe impl ::windows::core::Interface for HidNumericControl {
    const IID: ::windows::core::GUID = <IHidNumericControl as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidNumericControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidNumericControl";
}
::windows::core::interface_hierarchy!(HidNumericControl, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HidNumericControl {}
unsafe impl ::core::marker::Sync for HidNumericControl {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidNumericControlDescription(::windows::core::IUnknown);
impl HidNumericControlDescription {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReportId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReportType(&self) -> ::windows::core::Result<HidReportType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReportSize(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ReportCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsagePage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsageId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LogicalMinimum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LogicalMinimum)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LogicalMaximum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LogicalMaximum)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PhysicalMinimum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhysicalMinimum)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PhysicalMaximum(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhysicalMaximum)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UnitExponent(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnitExponent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Unit(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Unit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsAbsolute(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsAbsolute)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn HasNull(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasNull)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ParentCollections(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HidCollection>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ParentCollections)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HidNumericControlDescription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidNumericControlDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidNumericControlDescription {}
impl ::core::fmt::Debug for HidNumericControlDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidNumericControlDescription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidNumericControlDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription;{638d5e86-1d97-4c75-927f-5ff58ba05e32})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HidNumericControlDescription {
    type Vtable = IHidNumericControlDescription_Vtbl;
}
unsafe impl ::windows::core::Interface for HidNumericControlDescription {
    const IID: ::windows::core::GUID = <IHidNumericControlDescription as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidNumericControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription";
}
::windows::core::interface_hierarchy!(HidNumericControlDescription, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HidNumericControlDescription {}
unsafe impl ::core::marker::Sync for HidNumericControlDescription {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
pub struct HidOutputReport(::windows::core::IUnknown);
impl HidOutputReport {
    pub fn Id(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Data)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetData)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBooleanControl)(::windows::core::Vtable::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBooleanControlByDescription(&self, controldescription: &HidBooleanControlDescription) -> ::windows::core::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBooleanControlByDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(controldescription), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumericControl)(::windows::core::Vtable::as_raw(this), usagepage, usageid, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetNumericControlByDescription(&self, controldescription: &HidNumericControlDescription) -> ::windows::core::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetNumericControlByDescription)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(controldescription), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for HidOutputReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HidOutputReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HidOutputReport {}
impl ::core::fmt::Debug for HidOutputReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidOutputReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidOutputReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidOutputReport;{62cb2544-c896-4463-93c1-df9db053c450})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for HidOutputReport {
    type Vtable = IHidOutputReport_Vtbl;
}
unsafe impl ::windows::core::Interface for HidOutputReport {
    const IID: ::windows::core::GUID = <IHidOutputReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HidOutputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidOutputReport";
}
::windows::core::interface_hierarchy!(HidOutputReport, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for HidOutputReport {}
unsafe impl ::core::marker::Sync for HidOutputReport {}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HidCollectionType(pub i32);
impl HidCollectionType {
    pub const Physical: Self = Self(0i32);
    pub const Application: Self = Self(1i32);
    pub const Logical: Self = Self(2i32);
    pub const Report: Self = Self(3i32);
    pub const NamedArray: Self = Self(4i32);
    pub const UsageSwitch: Self = Self(5i32);
    pub const UsageModifier: Self = Self(6i32);
    pub const Other: Self = Self(7i32);
}
impl ::core::marker::Copy for HidCollectionType {}
impl ::core::clone::Clone for HidCollectionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HidCollectionType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HidCollectionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for HidCollectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidCollectionType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidCollectionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.HumanInterfaceDevice.HidCollectionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_HumanInterfaceDevice\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HidReportType(pub i32);
impl HidReportType {
    pub const Input: Self = Self(0i32);
    pub const Output: Self = Self(1i32);
    pub const Feature: Self = Self(2i32);
}
impl ::core::marker::Copy for HidReportType {}
impl ::core::clone::Clone for HidReportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HidReportType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HidReportType {
    type Abi = Self;
}
impl ::core::fmt::Debug for HidReportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HidReportType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HidReportType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.HumanInterfaceDevice.HidReportType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
