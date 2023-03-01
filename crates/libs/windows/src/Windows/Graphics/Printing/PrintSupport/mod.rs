#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportExtensionSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportExtensionSession {
    type Vtable = IPrintSupportExtensionSession_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportExtensionSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportExtensionSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea45f1a_f4c6_54b3_a0b8_a559839aa4c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
    #[cfg(feature = "Foundation")]
    pub PrintTicketValidationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintTicketValidationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrintTicketValidationRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrintTicketValidationRequested: usize,
    #[cfg(feature = "Foundation")]
    pub PrintDeviceCapabilitiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintDeviceCapabilitiesChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrintDeviceCapabilitiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrintDeviceCapabilitiesChanged: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportExtensionSession2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportExtensionSession2 {
    type Vtable = IPrintSupportExtensionSession2_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportExtensionSession2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportExtensionSession2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10fa8c11_6de8_5765_8fcf_e716e0f27ed1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionSession2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PrinterSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrinterSelected: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrinterSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrinterSelected: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportExtensionTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportExtensionTriggerDetails {
    type Vtable = IPrintSupportExtensionTriggerDetails_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportExtensionTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportExtensionTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae083711_9b09_55d1_a0ae_2a14c5f83d6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15969bf0_9028_5722_8a37_7d7c34b41dd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetCurrentPrintDeviceCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetCurrentPrintDeviceCapabilities: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub UpdatePrintDeviceCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatedpdc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    UpdatePrintDeviceCapabilities: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2 {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x469df9e7_fd07_5eeb_a07d_9fcc67f089ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetSupportedPdlPassthroughContentTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, supportedpdlcontenttypes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetSupportedPdlPassthroughContentTypes: usize,
    pub ResourceLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetCurrentPrintDeviceResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetCurrentPrintDeviceResources: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub UpdatePrintDeviceResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatedpdr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    UpdatePrintDeviceResources: usize,
    pub SetPrintDeviceCapabilitiesUpdatePolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatepolicy: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportPrintDeviceCapabilitiesUpdatePolicy(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesUpdatePolicy_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f5fc025_8c35_5529_8038_8cdc3634bbcd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesUpdatePolicy_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d9e1a70_7c39_551f_aa1f_f8ca35b3119e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreatePeriodicRefresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updateperiod: super::super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePeriodicRefresh: usize,
    pub CreatePrintJobRefresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numberofjobs: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportPrintTicketElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportPrintTicketElement {
    type Vtable = IPrintSupportPrintTicketElement_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportPrintTicketElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportPrintTicketElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b2a4489_730d_5be7_80e6_8332941abf13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintTicketElement_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub NamespaceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetNamespaceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportPrintTicketValidationRequestedEventArgs {
    type Vtable = IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportPrintTicketValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportPrintTicketValidationRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x338e4e69_db55_55c7_8338_ef64680a8f90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub PrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    PrintTicket: usize,
    pub SetPrintTicketValidationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: WorkflowPrintTicketValidationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportPrinterSelectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportPrinterSelectedEventArgs {
    type Vtable = IPrintSupportPrinterSelectedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportPrinterSelectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportPrinterSelectedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b1cb7d9_a8a4_5c09_adb2_66165f817977);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrinterSelectedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub SourceAppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    SourceAppInfo: usize,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub PrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    PrintTicket: usize,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub SetPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    SetPrintTicket: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAdditionalFeatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, features: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAdditionalFeatures: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAdditionalParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameters: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAdditionalParameters: usize,
    pub AllowedAdditionalFeaturesAndParametersCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Shell")]
    pub SetAdaptiveCard: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, adaptivecard: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Shell"))]
    SetAdaptiveCard: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportSessionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportSessionInfo {
    type Vtable = IPrintSupportSessionInfo_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportSessionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportSessionInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x852149af_777d_53e9_9ee9_45d3f4b5be9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSessionInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel")]
    pub SourceAppInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    SourceAppInfo: usize,
    #[cfg(feature = "Devices_Printers")]
    pub Printer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))]
    Printer: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportSettingsActivatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportSettingsActivatedEventArgs {
    type Vtable = IPrintSupportSettingsActivatedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportSettingsActivatedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e1b565e_a013_55ea_9b8c_eea39d9fb6c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSettingsActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportSettingsUISession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportSettingsUISession {
    type Vtable = IPrintSupportSettingsUISession_Vtbl;
}
impl ::core::clone::Clone for IPrintSupportSettingsUISession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintSupportSettingsUISession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6da2251_83c3_55e4_a0f8_5de8b062adbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSettingsUISession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub SessionPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    SessionPrintTicket: usize,
    pub DocumentTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LaunchKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SettingsLaunchKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub UpdatePrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticket: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    UpdatePrintTicket: usize,
    pub SessionInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportExtensionSession(::windows::core::IUnknown);
impl PrintSupportExtensionSession {
    #[doc = "*Required features: `\"Devices_Printers\"`*"]
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Devices::Printers::IppPrintDevice>();
            (::windows::core::Interface::vtable(this).Printer)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrintTicketValidationRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintTicketValidationRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PrintTicketValidationRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrintTicketValidationRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePrintTicketValidationRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrintDeviceCapabilitiesChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintDeviceCapabilitiesChangedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PrintDeviceCapabilitiesChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrintDeviceCapabilitiesChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePrintDeviceCapabilitiesChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrinterSelected(&self, handler: &super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrinterSelectedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::ComInterface::cast::<IPrintSupportExtensionSession2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).PrinterSelected)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrinterSelected(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintSupportExtensionSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePrinterSelected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
}
impl ::core::cmp::PartialEq for PrintSupportExtensionSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSupportExtensionSession {}
impl ::core::fmt::Debug for PrintSupportExtensionSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSupportExtensionSession").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintSupportExtensionSession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession;{eea45f1a-f4c6-54b3-a0b8-a559839aa4c3})");
}
impl ::core::clone::Clone for PrintSupportExtensionSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintSupportExtensionSession {
    type Vtable = IPrintSupportExtensionSession_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintSupportExtensionSession {
    const IID: ::windows::core::GUID = <IPrintSupportExtensionSession as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportExtensionSession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession";
}
::windows::imp::interface_hierarchy!(PrintSupportExtensionSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSupportExtensionSession {}
unsafe impl ::core::marker::Sync for PrintSupportExtensionSession {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportExtensionTriggerDetails(::windows::core::IUnknown);
impl PrintSupportExtensionTriggerDetails {
    pub fn Session(&self) -> ::windows::core::Result<PrintSupportExtensionSession> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintSupportExtensionSession>();
            (::windows::core::Interface::vtable(this).Session)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintSupportExtensionTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSupportExtensionTriggerDetails {}
impl ::core::fmt::Debug for PrintSupportExtensionTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSupportExtensionTriggerDetails").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintSupportExtensionTriggerDetails {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails;{ae083711-9b09-55d1-a0ae-2a14c5f83d6a})");
}
impl ::core::clone::Clone for PrintSupportExtensionTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintSupportExtensionTriggerDetails {
    type Vtable = IPrintSupportExtensionTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintSupportExtensionTriggerDetails {
    const IID: ::windows::core::GUID = <IPrintSupportExtensionTriggerDetails as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportExtensionTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails";
}
::windows::imp::interface_hierarchy!(PrintSupportExtensionTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSupportExtensionTriggerDetails {}
unsafe impl ::core::marker::Sync for PrintSupportExtensionTriggerDetails {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportPrintDeviceCapabilitiesChangedEventArgs(::windows::core::IUnknown);
impl PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetCurrentPrintDeviceCapabilities(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Data::Xml::Dom::XmlDocument>();
            (::windows::core::Interface::vtable(this).GetCurrentPrintDeviceCapabilities)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn UpdatePrintDeviceCapabilities(&self, updatedpdc: &super::super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UpdatePrintDeviceCapabilities)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(updatedpdc)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Deferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetSupportedPdlPassthroughContentTypes<P0>(&self, supportedpdlcontenttypes: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>,
    {
        let this = &::windows::core::ComInterface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSupportedPdlPassthroughContentTypes)(::windows::core::Interface::as_raw(this), supportedpdlcontenttypes.try_into_param()?.abi()).ok() }
    }
    pub fn ResourceLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::ComInterface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ResourceLanguage)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetCurrentPrintDeviceResources(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::ComInterface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Data::Xml::Dom::XmlDocument>();
            (::windows::core::Interface::vtable(this).GetCurrentPrintDeviceResources)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn UpdatePrintDeviceResources(&self, updatedpdr: &super::super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).UpdatePrintDeviceResources)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(updatedpdr)).ok() }
    }
    pub fn SetPrintDeviceCapabilitiesUpdatePolicy(&self, updatepolicy: &PrintSupportPrintDeviceCapabilitiesUpdatePolicy) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrintDeviceCapabilitiesUpdatePolicy)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(updatepolicy)).ok() }
    }
}
impl ::core::cmp::PartialEq for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
impl ::core::fmt::Debug for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSupportPrintDeviceCapabilitiesChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs;{15969bf0-9028-5722-8a37-7d7c34b41dd6})");
}
impl ::core::clone::Clone for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const IID: ::windows::core::GUID = <IPrintSupportPrintDeviceCapabilitiesChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs";
}
::windows::imp::interface_hierarchy!(PrintSupportPrintDeviceCapabilitiesChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportPrintDeviceCapabilitiesUpdatePolicy(::windows::core::IUnknown);
impl PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePeriodicRefresh(updateperiod: super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<PrintSupportPrintDeviceCapabilitiesUpdatePolicy> {
        Self::IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintSupportPrintDeviceCapabilitiesUpdatePolicy>();
            (::windows::core::Interface::vtable(this).CreatePeriodicRefresh)(::windows::core::Interface::as_raw(this), updateperiod, &mut result__).from_abi(result__)
        })
    }
    pub fn CreatePrintJobRefresh(numberofjobs: u32) -> ::windows::core::Result<PrintSupportPrintDeviceCapabilitiesUpdatePolicy> {
        Self::IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintSupportPrintDeviceCapabilitiesUpdatePolicy>();
            (::windows::core::Interface::vtable(this).CreatePrintJobRefresh)(::windows::core::Interface::as_raw(this), numberofjobs, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics<R, F: FnOnce(&IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PrintSupportPrintDeviceCapabilitiesUpdatePolicy, IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {}
impl ::core::fmt::Debug for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSupportPrintDeviceCapabilitiesUpdatePolicy").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy;{5f5fc025-8c35-5529-8038-8cdc3634bbcd})");
}
impl ::core::clone::Clone for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesUpdatePolicy_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const IID: ::windows::core::GUID = <IPrintSupportPrintDeviceCapabilitiesUpdatePolicy as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy";
}
::windows::imp::interface_hierarchy!(PrintSupportPrintDeviceCapabilitiesUpdatePolicy, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {}
unsafe impl ::core::marker::Sync for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportPrintTicketElement(::windows::core::IUnknown);
impl PrintSupportPrintTicketElement {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PrintSupportPrintTicketElement, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LocalName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLocalName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocalName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).NamespaceUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetNamespaceUri(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetNamespaceUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for PrintSupportPrintTicketElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSupportPrintTicketElement {}
impl ::core::fmt::Debug for PrintSupportPrintTicketElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSupportPrintTicketElement").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintSupportPrintTicketElement {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement;{4b2a4489-730d-5be7-80e6-8332941abf13})");
}
impl ::core::clone::Clone for PrintSupportPrintTicketElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintSupportPrintTicketElement {
    type Vtable = IPrintSupportPrintTicketElement_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintSupportPrintTicketElement {
    const IID: ::windows::core::GUID = <IPrintSupportPrintTicketElement as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportPrintTicketElement {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement";
}
::windows::imp::interface_hierarchy!(PrintSupportPrintTicketElement, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSupportPrintTicketElement {}
unsafe impl ::core::marker::Sync for PrintSupportPrintTicketElement {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportPrintTicketValidationRequestedEventArgs(::windows::core::IUnknown);
impl PrintSupportPrintTicketValidationRequestedEventArgs {
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn PrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::PrintTicket::WorkflowPrintTicket>();
            (::windows::core::Interface::vtable(this).PrintTicket)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPrintTicketValidationStatus(&self, status: WorkflowPrintTicketValidationStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrintTicketValidationStatus)(::windows::core::Interface::as_raw(this), status).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Deferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintSupportPrintTicketValidationRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSupportPrintTicketValidationRequestedEventArgs {}
impl ::core::fmt::Debug for PrintSupportPrintTicketValidationRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSupportPrintTicketValidationRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintSupportPrintTicketValidationRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs;{338e4e69-db55-55c7-8338-ef64680a8f90})");
}
impl ::core::clone::Clone for PrintSupportPrintTicketValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintSupportPrintTicketValidationRequestedEventArgs {
    type Vtable = IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintSupportPrintTicketValidationRequestedEventArgs {
    const IID: ::windows::core::GUID = <IPrintSupportPrintTicketValidationRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportPrintTicketValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(PrintSupportPrintTicketValidationRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSupportPrintTicketValidationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintSupportPrintTicketValidationRequestedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportPrinterSelectedEventArgs(::windows::core::IUnknown);
impl PrintSupportPrinterSelectedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn SourceAppInfo(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::ApplicationModel::AppInfo>();
            (::windows::core::Interface::vtable(this).SourceAppInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn PrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::PrintTicket::WorkflowPrintTicket>();
            (::windows::core::Interface::vtable(this).PrintTicket)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn SetPrintTicket(&self, value: &super::PrintTicket::WorkflowPrintTicket) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrintTicket)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAdditionalFeatures<P0>(&self, features: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<PrintSupportPrintTicketElement>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAdditionalFeatures)(::windows::core::Interface::as_raw(this), features.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAdditionalParameters<P0>(&self, parameters: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<PrintSupportPrintTicketElement>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAdditionalParameters)(::windows::core::Interface::as_raw(this), parameters.try_into_param()?.abi()).ok() }
    }
    pub fn AllowedAdditionalFeaturesAndParametersCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).AllowedAdditionalFeaturesAndParametersCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Shell\"`*"]
    #[cfg(feature = "UI_Shell")]
    pub fn SetAdaptiveCard<P0>(&self, adaptivecard: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::UI::Shell::IAdaptiveCard>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAdaptiveCard)(::windows::core::Interface::as_raw(this), adaptivecard.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Deferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintSupportPrinterSelectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSupportPrinterSelectedEventArgs {}
impl ::core::fmt::Debug for PrintSupportPrinterSelectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSupportPrinterSelectedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintSupportPrinterSelectedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs;{7b1cb7d9-a8a4-5c09-adb2-66165f817977})");
}
impl ::core::clone::Clone for PrintSupportPrinterSelectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintSupportPrinterSelectedEventArgs {
    type Vtable = IPrintSupportPrinterSelectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintSupportPrinterSelectedEventArgs {
    const IID: ::windows::core::GUID = <IPrintSupportPrinterSelectedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportPrinterSelectedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs";
}
::windows::imp::interface_hierarchy!(PrintSupportPrinterSelectedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSupportPrinterSelectedEventArgs {}
unsafe impl ::core::marker::Sync for PrintSupportPrinterSelectedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportSessionInfo(::windows::core::IUnknown);
impl PrintSupportSessionInfo {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn SourceAppInfo(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::ApplicationModel::AppInfo>();
            (::windows::core::Interface::vtable(this).SourceAppInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers\"`*"]
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Devices::Printers::IppPrintDevice>();
            (::windows::core::Interface::vtable(this).Printer)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintSupportSessionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSupportSessionInfo {}
impl ::core::fmt::Debug for PrintSupportSessionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSupportSessionInfo").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintSupportSessionInfo {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo;{852149af-777d-53e9-9ee9-45d3f4b5be9c})");
}
impl ::core::clone::Clone for PrintSupportSessionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintSupportSessionInfo {
    type Vtable = IPrintSupportSessionInfo_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintSupportSessionInfo {
    const IID: ::windows::core::GUID = <IPrintSupportSessionInfo as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportSessionInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo";
}
::windows::imp::interface_hierarchy!(PrintSupportSessionInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSupportSessionInfo {}
unsafe impl ::core::marker::Sync for PrintSupportSessionInfo {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportSettingsActivatedEventArgs(::windows::core::IUnknown);
impl PrintSupportSettingsActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::ApplicationModel::Activation::ActivationKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::ApplicationModel::Activation::ApplicationExecutionState>();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::ApplicationModel::Activation::SplashScreen>();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Session(&self) -> ::windows::core::Result<PrintSupportSettingsUISession> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintSupportSettingsUISession>();
            (::windows::core::Interface::vtable(this).Session)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Deferral>();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintSupportSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSupportSettingsActivatedEventArgs {}
impl ::core::fmt::Debug for PrintSupportSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSupportSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintSupportSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs;{1e1b565e-a013-55ea-9b8c-eea39d9fb6c1})");
}
impl ::core::clone::Clone for PrintSupportSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintSupportSettingsActivatedEventArgs {
    type Vtable = IPrintSupportSettingsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintSupportSettingsActivatedEventArgs {
    const IID: ::windows::core::GUID = <IPrintSupportSettingsActivatedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs";
}
::windows::imp::interface_hierarchy!(PrintSupportSettingsActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::CanTryInto<super::super::super::ApplicationModel::Activation::IActivatedEventArgs> for PrintSupportSettingsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::CanTryInto<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for PrintSupportSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Send for PrintSupportSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintSupportSettingsActivatedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportSettingsUISession(::windows::core::IUnknown);
impl PrintSupportSettingsUISession {
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn SessionPrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::PrintTicket::WorkflowPrintTicket>();
            (::windows::core::Interface::vtable(this).SessionPrintTicket)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DocumentTitle)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LaunchKind(&self) -> ::windows::core::Result<SettingsLaunchKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SettingsLaunchKind>();
            (::windows::core::Interface::vtable(this).LaunchKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn UpdatePrintTicket(&self, printticket: &super::PrintTicket::WorkflowPrintTicket) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UpdatePrintTicket)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(printticket)).ok() }
    }
    pub fn SessionInfo(&self) -> ::windows::core::Result<PrintSupportSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintSupportSessionInfo>();
            (::windows::core::Interface::vtable(this).SessionInfo)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintSupportSettingsUISession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintSupportSettingsUISession {}
impl ::core::fmt::Debug for PrintSupportSettingsUISession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintSupportSettingsUISession").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintSupportSettingsUISession {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession;{c6da2251-83c3-55e4-a0f8-5de8b062adbf})");
}
impl ::core::clone::Clone for PrintSupportSettingsUISession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintSupportSettingsUISession {
    type Vtable = IPrintSupportSettingsUISession_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintSupportSettingsUISession {
    const IID: ::windows::core::GUID = <IPrintSupportSettingsUISession as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportSettingsUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession";
}
::windows::imp::interface_hierarchy!(PrintSupportSettingsUISession, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSupportSettingsUISession {}
unsafe impl ::core::marker::Sync for PrintSupportSettingsUISession {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SettingsLaunchKind(pub i32);
impl SettingsLaunchKind {
    pub const JobPrintTicket: Self = Self(0i32);
    pub const UserDefaultPrintTicket: Self = Self(1i32);
}
impl ::core::marker::Copy for SettingsLaunchKind {}
impl ::core::clone::Clone for SettingsLaunchKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SettingsLaunchKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for SettingsLaunchKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for SettingsLaunchKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SettingsLaunchKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for SettingsLaunchKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.SettingsLaunchKind;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WorkflowPrintTicketValidationStatus(pub i32);
impl WorkflowPrintTicketValidationStatus {
    pub const Resolved: Self = Self(0i32);
    pub const Conflicting: Self = Self(1i32);
    pub const Invalid: Self = Self(2i32);
}
impl ::core::marker::Copy for WorkflowPrintTicketValidationStatus {}
impl ::core::clone::Clone for WorkflowPrintTicketValidationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WorkflowPrintTicketValidationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WorkflowPrintTicketValidationStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WorkflowPrintTicketValidationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WorkflowPrintTicketValidationStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WorkflowPrintTicketValidationStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.WorkflowPrintTicketValidationStatus;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
