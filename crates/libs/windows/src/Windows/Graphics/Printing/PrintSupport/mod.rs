#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportExtensionSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrintSupportExtensionSession {
    type Vtable = IPrintSupportExtensionSession_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportExtensionSession {
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
unsafe impl ::windows::core::Vtable for IPrintSupportExtensionSession2 {
    type Vtable = IPrintSupportExtensionSession2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportExtensionSession2 {
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
unsafe impl ::windows::core::Vtable for IPrintSupportExtensionTriggerDetails {
    type Vtable = IPrintSupportExtensionTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportExtensionTriggerDetails {
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
unsafe impl ::windows::core::Vtable for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
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
unsafe impl ::windows::core::Vtable for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2 {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2 {
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
    pub ResourceLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Vtable for IPrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesUpdatePolicy_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportPrintDeviceCapabilitiesUpdatePolicy {
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
unsafe impl ::windows::core::Vtable for IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics {
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
unsafe impl ::windows::core::Vtable for IPrintSupportPrintTicketElement {
    type Vtable = IPrintSupportPrintTicketElement_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportPrintTicketElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b2a4489_730d_5be7_80e6_8332941abf13);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintTicketElement_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub NamespaceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetNamespaceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPrintSupportPrintTicketValidationRequestedEventArgs {
    type Vtable = IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportPrintTicketValidationRequestedEventArgs {
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
unsafe impl ::windows::core::Vtable for IPrintSupportPrinterSelectedEventArgs {
    type Vtable = IPrintSupportPrinterSelectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportPrinterSelectedEventArgs {
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
unsafe impl ::windows::core::Vtable for IPrintSupportSessionInfo {
    type Vtable = IPrintSupportSessionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportSessionInfo {
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
unsafe impl ::windows::core::Vtable for IPrintSupportSettingsActivatedEventArgs {
    type Vtable = IPrintSupportSettingsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportSettingsActivatedEventArgs {
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
unsafe impl ::windows::core::Vtable for IPrintSupportSettingsUISession {
    type Vtable = IPrintSupportSettingsUISession_Vtbl;
}
unsafe impl ::windows::core::Interface for IPrintSupportSettingsUISession {
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
    pub DocumentTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Printer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrintTicketValidationRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintTicketValidationRequestedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrintTicketValidationRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrintTicketValidationRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePrintTicketValidationRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrintDeviceCapabilitiesChanged(&self, handler: &super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintDeviceCapabilitiesChangedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrintDeviceCapabilitiesChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrintDeviceCapabilitiesChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePrintDeviceCapabilitiesChanged)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrinterSelected(&self, handler: &super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrinterSelectedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IPrintSupportExtensionSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrinterSelected)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePrinterSelected(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintSupportExtensionSession2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePrinterSelected)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for PrintSupportExtensionSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintSupportExtensionSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession;{eea45f1a-f4c6-54b3-a0b8-a559839aa4c3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintSupportExtensionSession {
    type Vtable = IPrintSupportExtensionSession_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintSupportExtensionSession {
    const IID: ::windows::core::GUID = <IPrintSupportExtensionSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportExtensionSession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession";
}
::windows::core::interface_hierarchy!(PrintSupportExtensionSession, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSupportExtensionSession {}
unsafe impl ::core::marker::Sync for PrintSupportExtensionSession {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportExtensionTriggerDetails(::windows::core::IUnknown);
impl PrintSupportExtensionTriggerDetails {
    pub fn Session(&self) -> ::windows::core::Result<PrintSupportExtensionSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Session)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PrintSupportExtensionTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintSupportExtensionTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails;{ae083711-9b09-55d1-a0ae-2a14c5f83d6a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintSupportExtensionTriggerDetails {
    type Vtable = IPrintSupportExtensionTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintSupportExtensionTriggerDetails {
    const IID: ::windows::core::GUID = <IPrintSupportExtensionTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportExtensionTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails";
}
::windows::core::interface_hierarchy!(PrintSupportExtensionTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentPrintDeviceCapabilities)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn UpdatePrintDeviceCapabilities(&self, updatedpdc: &super::super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdatePrintDeviceCapabilities)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(updatedpdc)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetSupportedPdlPassthroughContentTypes<P0, E0>(&self, supportedpdlcontenttypes: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSupportedPdlPassthroughContentTypes)(::windows::core::Vtable::as_raw(this), supportedpdlcontenttypes.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn ResourceLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ResourceLanguage)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetCurrentPrintDeviceResources(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::XmlDocument> {
        let this = &::windows::core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetCurrentPrintDeviceResources)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn UpdatePrintDeviceResources(&self, updatedpdr: &super::super::super::Data::Xml::Dom::XmlDocument) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).UpdatePrintDeviceResources)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(updatedpdr)).ok() }
    }
    pub fn SetPrintDeviceCapabilitiesUpdatePolicy(&self, updatepolicy: &PrintSupportPrintDeviceCapabilitiesUpdatePolicy) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPrintSupportPrintDeviceCapabilitiesChangedEventArgs2>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrintDeviceCapabilitiesUpdatePolicy)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(updatepolicy)).ok() }
    }
}
impl ::core::clone::Clone for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs;{15969bf0-9028-5722-8a37-7d7c34b41dd6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const IID: ::windows::core::GUID = <IPrintSupportPrintDeviceCapabilitiesChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs";
}
::windows::core::interface_hierarchy!(PrintSupportPrintDeviceCapabilitiesChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreatePeriodicRefresh)(::windows::core::Vtable::as_raw(this), updateperiod, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreatePrintJobRefresh(numberofjobs: u32) -> ::windows::core::Result<PrintSupportPrintDeviceCapabilitiesUpdatePolicy> {
        Self::IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreatePrintJobRefresh)(::windows::core::Vtable::as_raw(this), numberofjobs, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics<R, F: FnOnce(&IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PrintSupportPrintDeviceCapabilitiesUpdatePolicy, IPrintSupportPrintDeviceCapabilitiesUpdatePolicyStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy;{5f5fc025-8c35-5529-8038-8cdc3634bbcd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesUpdatePolicy_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const IID: ::windows::core::GUID = <IPrintSupportPrintDeviceCapabilitiesUpdatePolicy as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesUpdatePolicy";
}
::windows::core::interface_hierarchy!(PrintSupportPrintDeviceCapabilitiesUpdatePolicy, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {}
unsafe impl ::core::marker::Sync for PrintSupportPrintDeviceCapabilitiesUpdatePolicy {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportPrintTicketElement(::windows::core::IUnknown);
impl PrintSupportPrintTicketElement {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PrintSupportPrintTicketElement, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn LocalName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LocalName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLocalName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLocalName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn NamespaceUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).NamespaceUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetNamespaceUri(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetNamespaceUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::clone::Clone for PrintSupportPrintTicketElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintSupportPrintTicketElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement;{4b2a4489-730d-5be7-80e6-8332941abf13})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintSupportPrintTicketElement {
    type Vtable = IPrintSupportPrintTicketElement_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintSupportPrintTicketElement {
    const IID: ::windows::core::GUID = <IPrintSupportPrintTicketElement as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportPrintTicketElement {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketElement";
}
::windows::core::interface_hierarchy!(PrintSupportPrintTicketElement, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrintTicket)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPrintTicketValidationStatus(&self, status: WorkflowPrintTicketValidationStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrintTicketValidationStatus)(::windows::core::Vtable::as_raw(this), status).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PrintSupportPrintTicketValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintSupportPrintTicketValidationRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs;{338e4e69-db55-55c7-8338-ef64680a8f90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintSupportPrintTicketValidationRequestedEventArgs {
    type Vtable = IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintSupportPrintTicketValidationRequestedEventArgs {
    const IID: ::windows::core::GUID = <IPrintSupportPrintTicketValidationRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportPrintTicketValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs";
}
::windows::core::interface_hierarchy!(PrintSupportPrintTicketValidationRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourceAppInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn PrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrintTicket)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn SetPrintTicket(&self, value: &super::PrintTicket::WorkflowPrintTicket) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPrintTicket)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAdditionalFeatures<P0, E0>(&self, features: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<PrintSupportPrintTicketElement>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAdditionalFeatures)(::windows::core::Vtable::as_raw(this), features.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAdditionalParameters<P0, E0>(&self, parameters: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::Foundation::Collections::IIterable<PrintSupportPrintTicketElement>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAdditionalParameters)(::windows::core::Vtable::as_raw(this), parameters.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn AllowedAdditionalFeaturesAndParametersCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AllowedAdditionalFeaturesAndParametersCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Shell\"`*"]
    #[cfg(feature = "UI_Shell")]
    pub fn SetAdaptiveCard<P0, E0>(&self, adaptivecard: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::super::UI::Shell::IAdaptiveCard>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetAdaptiveCard)(::windows::core::Vtable::as_raw(this), adaptivecard.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PrintSupportPrinterSelectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintSupportPrinterSelectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs;{7b1cb7d9-a8a4-5c09-adb2-66165f817977})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintSupportPrinterSelectedEventArgs {
    type Vtable = IPrintSupportPrinterSelectedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintSupportPrinterSelectedEventArgs {
    const IID: ::windows::core::GUID = <IPrintSupportPrinterSelectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportPrinterSelectedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrinterSelectedEventArgs";
}
::windows::core::interface_hierarchy!(PrintSupportPrinterSelectedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SourceAppInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers\"`*"]
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Printer)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PrintSupportSessionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintSupportSessionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo;{852149af-777d-53e9-9ee9-45d3f4b5be9c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintSupportSessionInfo {
    type Vtable = IPrintSupportSessionInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintSupportSessionInfo {
    const IID: ::windows::core::GUID = <IPrintSupportSessionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportSessionInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo";
}
::windows::core::interface_hierarchy!(PrintSupportSessionInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintSupportSessionInfo {}
unsafe impl ::core::marker::Sync for PrintSupportSessionInfo {}
#[doc = "*Required features: `\"Graphics_Printing_PrintSupport\"`*"]
#[repr(transparent)]
pub struct PrintSupportSettingsActivatedEventArgs(::windows::core::IUnknown);
impl PrintSupportSettingsActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Kind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PreviousExecutionState)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SplashScreen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).User)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Session(&self) -> ::windows::core::Result<PrintSupportSettingsUISession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Session)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PrintSupportSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintSupportSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs;{1e1b565e-a013-55ea-9b8c-eea39d9fb6c1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintSupportSettingsActivatedEventArgs {
    type Vtable = IPrintSupportSettingsActivatedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintSupportSettingsActivatedEventArgs {
    const IID: ::windows::core::GUID = <IPrintSupportSettingsActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs";
}
::windows::core::interface_hierarchy!(PrintSupportSettingsActivatedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<PrintSupportSettingsActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintSupportSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&PrintSupportSettingsActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintSupportSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&PrintSupportSettingsActivatedEventArgs> for ::windows::core::InParam<super::super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintSupportSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<PrintSupportSettingsActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: PrintSupportSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&PrintSupportSettingsActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintSupportSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&PrintSupportSettingsActivatedEventArgs> for ::windows::core::InParam<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PrintSupportSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
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
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionPrintTicket)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DocumentTitle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LaunchKind(&self) -> ::windows::core::Result<SettingsLaunchKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LaunchKind)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn UpdatePrintTicket(&self, printticket: &super::PrintTicket::WorkflowPrintTicket) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).UpdatePrintTicket)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(printticket)).ok() }
    }
    pub fn SessionInfo(&self) -> ::windows::core::Result<PrintSupportSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SessionInfo)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for PrintSupportSettingsUISession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintSupportSettingsUISession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession;{c6da2251-83c3-55e4-a0f8-5de8b062adbf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PrintSupportSettingsUISession {
    type Vtable = IPrintSupportSettingsUISession_Vtbl;
}
unsafe impl ::windows::core::Interface for PrintSupportSettingsUISession {
    const IID: ::windows::core::GUID = <IPrintSupportSettingsUISession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportSettingsUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession";
}
::windows::core::interface_hierarchy!(PrintSupportSettingsUISession, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
unsafe impl ::windows::core::Abi for SettingsLaunchKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for SettingsLaunchKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SettingsLaunchKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SettingsLaunchKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.SettingsLaunchKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for WorkflowPrintTicketValidationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for WorkflowPrintTicketValidationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WorkflowPrintTicketValidationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WorkflowPrintTicketValidationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.WorkflowPrintTicketValidationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
