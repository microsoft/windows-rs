#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintSupportExtensionSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintSupportExtensionSession {
    type Vtable = IPrintSupportExtensionSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4003749658, 62662, 21683, [160, 184, 165, 89, 131, 154, 164, 195]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Printers")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintSupportExtensionTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintSupportExtensionTriggerDetails {
    type Vtable = IPrintSupportExtensionTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2919773969, 39689, 21969, [160, 174, 42, 20, 197, 248, 61, 106]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionTriggerDetails_abi(
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
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(362191856, 36904, 22306, [138, 55, 125, 124, 52, 180, 29, 214]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updatedpdc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintSupportPrintTicketValidationRequestedEventArgs {
    type Vtable = IPrintSupportPrintTicketValidationRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(864964201, 56149, 21959, [131, 56, 239, 100, 104, 10, 143, 144]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: WorkflowPrintTicketValidationStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintSupportSessionInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintSupportSessionInfo {
    type Vtable = IPrintSupportSessionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2233551279, 30589, 21481, [158, 233, 69, 211, 244, 181, 190, 156]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSessionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
    #[cfg(feature = "Devices_Printers")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintSupportSettingsActivatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintSupportSettingsActivatedEventArgs {
    type Vtable = IPrintSupportSettingsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(505108062, 40979, 21994, [155, 140, 238, 163, 157, 159, 182, 193]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSettingsActivatedEventArgs_abi(
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
pub struct IPrintSupportSettingsUISession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPrintSupportSettingsUISession {
    type Vtable = IPrintSupportSettingsUISession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3336184401, 33731, 21988, [160, 248, 93, 232, 176, 98, 173, 191]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSettingsUISession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SettingsLaunchKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printticket: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PrintSupportExtensionSession(::windows::runtime::IInspectable);
impl PrintSupportExtensionSession {
    #[cfg(feature = "Devices_Printers")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Devices_Printers`*"]
    pub fn Printer(&self) -> ::windows::runtime::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Printers::IppPrintDevice>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Foundation`*"]
    pub fn PrintTicketValidationRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintTicketValidationRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Foundation`*"]
    pub fn RemovePrintTicketValidationRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Foundation`*"]
    pub fn PrintDeviceCapabilitiesChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintDeviceCapabilitiesChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Foundation`*"]
    pub fn RemovePrintDeviceCapabilitiesChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintSupportExtensionSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession;{eea45f1a-f4c6-54b3-a0b8-a559839aa4c3})");
}
unsafe impl ::windows::runtime::Interface for PrintSupportExtensionSession {
    type Vtable = IPrintSupportExtensionSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4003749658, 62662, 21683, [160, 184, 165, 89, 131, 154, 164, 195]);
}
impl ::windows::runtime::RuntimeName for PrintSupportExtensionSession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession";
}
unsafe impl ::std::marker::Send for PrintSupportExtensionSession {}
unsafe impl ::std::marker::Sync for PrintSupportExtensionSession {}
#[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PrintSupportExtensionTriggerDetails(::windows::runtime::IInspectable);
impl PrintSupportExtensionTriggerDetails {
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
    pub fn Session(&self) -> ::windows::runtime::Result<PrintSupportExtensionSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PrintSupportExtensionSession>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintSupportExtensionTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails;{ae083711-9b09-55d1-a0ae-2a14c5f83d6a})");
}
unsafe impl ::windows::runtime::Interface for PrintSupportExtensionTriggerDetails {
    type Vtable = IPrintSupportExtensionTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2919773969, 39689, 21969, [160, 174, 42, 20, 197, 248, 61, 106]);
}
impl ::windows::runtime::RuntimeName for PrintSupportExtensionTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails";
}
unsafe impl ::std::marker::Send for PrintSupportExtensionTriggerDetails {}
unsafe impl ::std::marker::Sync for PrintSupportExtensionTriggerDetails {}
#[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PrintSupportPrintDeviceCapabilitiesChangedEventArgs(::windows::runtime::IInspectable);
impl PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Data_Xml_Dom`*"]
    pub fn GetCurrentPrintDeviceCapabilities(&self) -> ::windows::runtime::Result<super::super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Data_Xml_Dom`*"]
    pub fn UpdatePrintDeviceCapabilities<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Data::Xml::Dom::XmlDocument>>(&self, updatedpdc: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), updatedpdc.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs;{15969bf0-9028-5722-8a37-7d7c34b41dd6})");
}
unsafe impl ::windows::runtime::Interface for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(362191856, 36904, 22306, [138, 55, 125, 124, 52, 180, 29, 214]);
}
impl ::windows::runtime::RuntimeName for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs";
}
unsafe impl ::std::marker::Send for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
unsafe impl ::std::marker::Sync for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
#[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PrintSupportPrintTicketValidationRequestedEventArgs(::windows::runtime::IInspectable);
impl PrintSupportPrintTicketValidationRequestedEventArgs {
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Graphics_Printing_PrintTicket`*"]
    pub fn PrintTicket(&self) -> ::windows::runtime::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::PrintTicket::WorkflowPrintTicket>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
    pub fn SetPrintTicketValidationStatus(&self, status: WorkflowPrintTicketValidationStatus) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), status).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintSupportPrintTicketValidationRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs;{338e4e69-db55-55c7-8338-ef64680a8f90})");
}
unsafe impl ::windows::runtime::Interface for PrintSupportPrintTicketValidationRequestedEventArgs {
    type Vtable = IPrintSupportPrintTicketValidationRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(864964201, 56149, 21959, [131, 56, 239, 100, 104, 10, 143, 144]);
}
impl ::windows::runtime::RuntimeName for PrintSupportPrintTicketValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs";
}
unsafe impl ::std::marker::Send for PrintSupportPrintTicketValidationRequestedEventArgs {}
unsafe impl ::std::marker::Sync for PrintSupportPrintTicketValidationRequestedEventArgs {}
#[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PrintSupportSessionInfo(::windows::runtime::IInspectable);
impl PrintSupportSessionInfo {
    #[cfg(feature = "ApplicationModel")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `ApplicationModel`*"]
    pub fn SourceAppInfo(&self) -> ::windows::runtime::Result<super::super::super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::AppInfo>(result__)
        }
    }
    #[cfg(feature = "Devices_Printers")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Devices_Printers`*"]
    pub fn Printer(&self) -> ::windows::runtime::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Printers::IppPrintDevice>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintSupportSessionInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo;{852149af-777d-53e9-9ee9-45d3f4b5be9c})");
}
unsafe impl ::windows::runtime::Interface for PrintSupportSessionInfo {
    type Vtable = IPrintSupportSessionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2233551279, 30589, 21481, [158, 233, 69, 211, 244, 181, 190, 156]);
}
impl ::windows::runtime::RuntimeName for PrintSupportSessionInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo";
}
unsafe impl ::std::marker::Send for PrintSupportSessionInfo {}
unsafe impl ::std::marker::Sync for PrintSupportSessionInfo {}
#[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PrintSupportSettingsActivatedEventArgs(::windows::runtime::IInspectable);
impl PrintSupportSettingsActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `ApplicationModel_Activation`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<super::super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::super::ApplicationModel::Activation::ActivationKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `ApplicationModel_Activation`*"]
    pub fn PreviousExecutionState(&self) -> ::windows::runtime::Result<super::super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::super::ApplicationModel::Activation::ApplicationExecutionState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `ApplicationModel_Activation`*"]
    pub fn SplashScreen(&self) -> ::windows::runtime::Result<super::super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `ApplicationModel_Activation`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
    pub fn Session(&self) -> ::windows::runtime::Result<PrintSupportSettingsUISession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PrintSupportSettingsUISession>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintSupportSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs;{1e1b565e-a013-55ea-9b8c-eea39d9fb6c1})");
}
unsafe impl ::windows::runtime::Interface for PrintSupportSettingsActivatedEventArgs {
    type Vtable = IPrintSupportSettingsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(505108062, 40979, 21994, [155, 140, 238, 163, 157, 159, 182, 193]);
}
impl ::windows::runtime::RuntimeName for PrintSupportSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<PrintSupportSettingsActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PrintSupportSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&PrintSupportSettingsActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PrintSupportSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> for PrintSupportSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> for &PrintSupportSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::std::convert::TryInto::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<PrintSupportSettingsActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PrintSupportSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&PrintSupportSettingsActivatedEventArgs> for super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PrintSupportSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for PrintSupportSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for &PrintSupportSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
        ::std::convert::TryInto::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for PrintSupportSettingsActivatedEventArgs {}
unsafe impl ::std::marker::Sync for PrintSupportSettingsActivatedEventArgs {}
#[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PrintSupportSettingsUISession(::windows::runtime::IInspectable);
impl PrintSupportSettingsUISession {
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Graphics_Printing_PrintTicket`*"]
    pub fn SessionPrintTicket(&self) -> ::windows::runtime::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::PrintTicket::WorkflowPrintTicket>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
    pub fn DocumentTitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
    pub fn LaunchKind(&self) -> ::windows::runtime::Result<SettingsLaunchKind> {
        let this = self;
        unsafe {
            let mut result__: SettingsLaunchKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SettingsLaunchKind>(result__)
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`, `Graphics_Printing_PrintTicket`*"]
    pub fn UpdatePrintTicket<'a, Param0: ::windows::runtime::IntoParam<'a, super::PrintTicket::WorkflowPrintTicket>>(&self, printticket: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), printticket.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
    pub fn SessionInfo(&self) -> ::windows::runtime::Result<PrintSupportSessionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PrintSupportSessionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PrintSupportSettingsUISession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession;{c6da2251-83c3-55e4-a0f8-5de8b062adbf})");
}
unsafe impl ::windows::runtime::Interface for PrintSupportSettingsUISession {
    type Vtable = IPrintSupportSettingsUISession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3336184401, 33731, 21988, [160, 248, 93, 232, 176, 98, 173, 191]);
}
impl ::windows::runtime::RuntimeName for PrintSupportSettingsUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession";
}
unsafe impl ::std::marker::Send for PrintSupportSettingsUISession {}
unsafe impl ::std::marker::Sync for PrintSupportSettingsUISession {}
#[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SettingsLaunchKind(pub i32);
impl SettingsLaunchKind {
    pub const JobPrintTicket: SettingsLaunchKind = SettingsLaunchKind(0i32);
    pub const UserDefaultPrintTicket: SettingsLaunchKind = SettingsLaunchKind(1i32);
}
impl ::std::convert::From<i32> for SettingsLaunchKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SettingsLaunchKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SettingsLaunchKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.SettingsLaunchKind;i4)");
}
impl ::windows::runtime::DefaultType for SettingsLaunchKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_Printing_PrintSupport`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WorkflowPrintTicketValidationStatus(pub i32);
impl WorkflowPrintTicketValidationStatus {
    pub const Resolved: WorkflowPrintTicketValidationStatus = WorkflowPrintTicketValidationStatus(0i32);
    pub const Conflicting: WorkflowPrintTicketValidationStatus = WorkflowPrintTicketValidationStatus(1i32);
    pub const Invalid: WorkflowPrintTicketValidationStatus = WorkflowPrintTicketValidationStatus(2i32);
}
impl ::std::convert::From<i32> for WorkflowPrintTicketValidationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WorkflowPrintTicketValidationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WorkflowPrintTicketValidationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.WorkflowPrintTicketValidationStatus;i4)");
}
impl ::windows::runtime::DefaultType for WorkflowPrintTicketValidationStatus {
    type DefaultType = Self;
}
