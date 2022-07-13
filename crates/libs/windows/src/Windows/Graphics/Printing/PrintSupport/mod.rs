#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportExtensionSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportExtensionSession {
    type Vtable = IPrintSupportExtensionSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea45f1a_f4c6_54b3_a0b8_a559839aa4c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionSession_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
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
pub struct IPrintSupportExtensionTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportExtensionTriggerDetails {
    type Vtable = IPrintSupportExtensionTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae083711_9b09_55d1_a0ae_2a14c5f83d6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15969bf0_9028_5722_8a37_7d7c34b41dd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
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
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportPrintTicketValidationRequestedEventArgs {
    type Vtable = IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x338e4e69_db55_55c7_8338_ef64680a8f90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
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
pub struct IPrintSupportSessionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintSupportSessionInfo {
    type Vtable = IPrintSupportSessionInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x852149af_777d_53e9_9ee9_45d3f4b5be9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSessionInfo_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e1b565e_a013_55ea_9b8c_eea39d9fb6c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSettingsActivatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6da2251_83c3_55e4_a0f8_5de8b062adbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSettingsUISession_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub SessionPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))]
    SessionPrintTicket: usize,
    pub DocumentTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
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
            (::windows::core::Interface::vtable(this).Printer)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Devices::Printers::IppPrintDevice>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrintTicketValidationRequested<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintTicketValidationRequestedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrintTicketValidationRequested)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
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
    pub fn PrintDeviceCapabilitiesChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintDeviceCapabilitiesChangedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrintDeviceCapabilitiesChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
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
unsafe impl ::windows::core::Interface for PrintSupportExtensionSession {
    type Vtable = IPrintSupportExtensionSession_Vtbl;
    const IID: ::windows::core::GUID = <IPrintSupportExtensionSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportExtensionSession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession";
}
impl ::core::convert::From<PrintSupportExtensionSession> for ::windows::core::IUnknown {
    fn from(value: PrintSupportExtensionSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportExtensionSession> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportExtensionSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportExtensionSession> for &::windows::core::IUnknown {
    fn from(value: &PrintSupportExtensionSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintSupportExtensionSession> for ::windows::core::IInspectable {
    fn from(value: PrintSupportExtensionSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportExtensionSession> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportExtensionSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportExtensionSession> for &::windows::core::IInspectable {
    fn from(value: &PrintSupportExtensionSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
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
            (::windows::core::Interface::vtable(this).Session)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintSupportExtensionSession>(result__)
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
unsafe impl ::windows::core::Interface for PrintSupportExtensionTriggerDetails {
    type Vtable = IPrintSupportExtensionTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IPrintSupportExtensionTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportExtensionTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails";
}
impl ::core::convert::From<PrintSupportExtensionTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PrintSupportExtensionTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportExtensionTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportExtensionTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportExtensionTriggerDetails> for &::windows::core::IUnknown {
    fn from(value: &PrintSupportExtensionTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintSupportExtensionTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PrintSupportExtensionTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportExtensionTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportExtensionTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportExtensionTriggerDetails> for &::windows::core::IInspectable {
    fn from(value: &PrintSupportExtensionTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
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
            (::windows::core::Interface::vtable(this).GetCurrentPrintDeviceCapabilities)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn UpdatePrintDeviceCapabilities<'a, P0>(&self, updatedpdc: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Data::Xml::Dom::XmlDocument>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UpdatePrintDeviceCapabilities)(::windows::core::Interface::as_raw(this), updatedpdc.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
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
unsafe impl ::windows::core::Interface for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintSupportPrintDeviceCapabilitiesChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs";
}
impl ::core::convert::From<PrintSupportPrintDeviceCapabilitiesChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintSupportPrintDeviceCapabilitiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportPrintDeviceCapabilitiesChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportPrintDeviceCapabilitiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportPrintDeviceCapabilitiesChangedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PrintSupportPrintDeviceCapabilitiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintSupportPrintDeviceCapabilitiesChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintSupportPrintDeviceCapabilitiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportPrintDeviceCapabilitiesChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportPrintDeviceCapabilitiesChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportPrintDeviceCapabilitiesChangedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PrintSupportPrintDeviceCapabilitiesChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
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
            (::windows::core::Interface::vtable(this).PrintTicket)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::PrintTicket::WorkflowPrintTicket>(result__)
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
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
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
unsafe impl ::windows::core::Interface for PrintSupportPrintTicketValidationRequestedEventArgs {
    type Vtable = IPrintSupportPrintTicketValidationRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintSupportPrintTicketValidationRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportPrintTicketValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs";
}
impl ::core::convert::From<PrintSupportPrintTicketValidationRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintSupportPrintTicketValidationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportPrintTicketValidationRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportPrintTicketValidationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportPrintTicketValidationRequestedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PrintSupportPrintTicketValidationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintSupportPrintTicketValidationRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintSupportPrintTicketValidationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportPrintTicketValidationRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportPrintTicketValidationRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportPrintTicketValidationRequestedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PrintSupportPrintTicketValidationRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PrintSupportPrintTicketValidationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintSupportPrintTicketValidationRequestedEventArgs {}
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
            (::windows::core::Interface::vtable(this).SourceAppInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::ApplicationModel::AppInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_Printers\"`*"]
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Printer)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Devices::Printers::IppPrintDevice>(result__)
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
unsafe impl ::windows::core::Interface for PrintSupportSessionInfo {
    type Vtable = IPrintSupportSessionInfo_Vtbl;
    const IID: ::windows::core::GUID = <IPrintSupportSessionInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportSessionInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo";
}
impl ::core::convert::From<PrintSupportSessionInfo> for ::windows::core::IUnknown {
    fn from(value: PrintSupportSessionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportSessionInfo> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportSessionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportSessionInfo> for &::windows::core::IUnknown {
    fn from(value: &PrintSupportSessionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintSupportSessionInfo> for ::windows::core::IInspectable {
    fn from(value: PrintSupportSessionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportSessionInfo> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportSessionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportSessionInfo> for &::windows::core::IInspectable {
    fn from(value: &PrintSupportSessionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
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
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::System::User>(result__)
        }
    }
    pub fn Session(&self) -> ::windows::core::Result<PrintSupportSettingsUISession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Session)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintSupportSettingsUISession>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Deferral>(result__)
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
unsafe impl ::windows::core::Interface for PrintSupportSettingsActivatedEventArgs {
    type Vtable = IPrintSupportSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPrintSupportSettingsActivatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs";
}
impl ::core::convert::From<PrintSupportSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintSupportSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportSettingsActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PrintSupportSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintSupportSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintSupportSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportSettingsActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PrintSupportSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
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
impl<'a> ::core::convert::TryFrom<&PrintSupportSettingsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> {
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
impl<'a> ::core::convert::TryFrom<&PrintSupportSettingsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
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
            (::windows::core::Interface::vtable(this).SessionPrintTicket)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::PrintTicket::WorkflowPrintTicket>(result__)
        }
    }
    pub fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DocumentTitle)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn LaunchKind(&self) -> ::windows::core::Result<SettingsLaunchKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LaunchKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<SettingsLaunchKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn UpdatePrintTicket<'a, P0>(&self, printticket: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::PrintTicket::WorkflowPrintTicket>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UpdatePrintTicket)(::windows::core::Interface::as_raw(this), printticket.into().abi()).ok() }
    }
    pub fn SessionInfo(&self) -> ::windows::core::Result<PrintSupportSessionInfo> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SessionInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintSupportSessionInfo>(result__)
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
unsafe impl ::windows::core::Interface for PrintSupportSettingsUISession {
    type Vtable = IPrintSupportSettingsUISession_Vtbl;
    const IID: ::windows::core::GUID = <IPrintSupportSettingsUISession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintSupportSettingsUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession";
}
impl ::core::convert::From<PrintSupportSettingsUISession> for ::windows::core::IUnknown {
    fn from(value: PrintSupportSettingsUISession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportSettingsUISession> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportSettingsUISession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportSettingsUISession> for &::windows::core::IUnknown {
    fn from(value: &PrintSupportSettingsUISession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PrintSupportSettingsUISession> for ::windows::core::IInspectable {
    fn from(value: PrintSupportSettingsUISession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintSupportSettingsUISession> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportSettingsUISession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PrintSupportSettingsUISession> for &::windows::core::IInspectable {
    fn from(value: &PrintSupportSettingsUISession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
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
