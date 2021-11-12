#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintSupportExtensionSession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintSupportExtensionSession {
    type Vtable = IPrintSupportExtensionSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea45f1a_f4c6_54b3_a0b8_a559839aa4c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Devices_Printers")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintSupportExtensionTriggerDetails(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintSupportExtensionTriggerDetails {
    type Vtable = IPrintSupportExtensionTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae083711_9b09_55d1_a0ae_2a14c5f83d6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportExtensionTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15969bf0_9028_5722_8a37_7d7c34b41dd6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Data_Xml_Dom")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updatedpdc: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintSupportPrintTicketValidationRequestedEventArgs {
    type Vtable = IPrintSupportPrintTicketValidationRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x338e4e69_db55_55c7_8338_ef64680a8f90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportPrintTicketValidationRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, status: WorkflowPrintTicketValidationStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintSupportSessionInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintSupportSessionInfo {
    type Vtable = IPrintSupportSessionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x852149af_777d_53e9_9ee9_45d3f4b5be9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSessionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))] usize,
    #[cfg(feature = "Devices_Printers")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Devices_Printers"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintSupportSettingsActivatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintSupportSettingsActivatedEventArgs {
    type Vtable = IPrintSupportSettingsActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e1b565e_a013_55ea_9b8c_eea39d9fb6c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSettingsActivatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPrintSupportSettingsUISession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPrintSupportSettingsUISession {
    type Vtable = IPrintSupportSettingsUISession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6da2251_83c3_55e4_a0f8_5de8b062adbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintSupportSettingsUISession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SettingsLaunchKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Printing_PrintTicket")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, printticket: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing_PrintTicket"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintSupportExtensionSession(pub ::windows::core::IInspectable);
impl PrintSupportExtensionSession {
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Printers::IppPrintDevice>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn PrintTicketValidationRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintTicketValidationRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePrintTicketValidationRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PrintDeviceCapabilitiesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<PrintSupportExtensionSession, PrintSupportPrintDeviceCapabilitiesChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePrintDeviceCapabilitiesChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintSupportExtensionSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession;{eea45f1a-f4c6-54b3-a0b8-a559839aa4c3})");
}
unsafe impl ::windows::core::Interface for PrintSupportExtensionSession {
    type Vtable = IPrintSupportExtensionSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea45f1a_f4c6_54b3_a0b8_a559839aa4c3);
}
impl ::windows::core::RuntimeName for PrintSupportExtensionSession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionSession";
}
impl ::core::convert::From<PrintSupportExtensionSession> for ::windows::core::IUnknown {
    fn from(value: PrintSupportExtensionSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintSupportExtensionSession> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportExtensionSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintSupportExtensionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintSupportExtensionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintSupportExtensionSession> for ::windows::core::IInspectable {
    fn from(value: PrintSupportExtensionSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintSupportExtensionSession> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportExtensionSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintSupportExtensionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintSupportExtensionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintSupportExtensionSession {}
unsafe impl ::core::marker::Sync for PrintSupportExtensionSession {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintSupportExtensionTriggerDetails(pub ::windows::core::IInspectable);
impl PrintSupportExtensionTriggerDetails {
    pub fn Session(&self) -> ::windows::core::Result<PrintSupportExtensionSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintSupportExtensionSession>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintSupportExtensionTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails;{ae083711-9b09-55d1-a0ae-2a14c5f83d6a})");
}
unsafe impl ::windows::core::Interface for PrintSupportExtensionTriggerDetails {
    type Vtable = IPrintSupportExtensionTriggerDetails_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae083711_9b09_55d1_a0ae_2a14c5f83d6a);
}
impl ::windows::core::RuntimeName for PrintSupportExtensionTriggerDetails {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportExtensionTriggerDetails";
}
impl ::core::convert::From<PrintSupportExtensionTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: PrintSupportExtensionTriggerDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintSupportExtensionTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportExtensionTriggerDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintSupportExtensionTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintSupportExtensionTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintSupportExtensionTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: PrintSupportExtensionTriggerDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintSupportExtensionTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportExtensionTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintSupportExtensionTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintSupportExtensionTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintSupportExtensionTriggerDetails {}
unsafe impl ::core::marker::Sync for PrintSupportExtensionTriggerDetails {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintSupportPrintDeviceCapabilitiesChangedEventArgs(pub ::windows::core::IInspectable);
impl PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetCurrentPrintDeviceCapabilities(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::XmlDocument> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Data::Xml::Dom::XmlDocument>(result__)
        }
    }
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn UpdatePrintDeviceCapabilities<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Data::Xml::Dom::XmlDocument>>(&self, updatedpdc: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), updatedpdc.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs;{15969bf0-9028-5722-8a37-7d7c34b41dd6})");
}
unsafe impl ::windows::core::Interface for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    type Vtable = IPrintSupportPrintDeviceCapabilitiesChangedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15969bf0_9028_5722_8a37_7d7c34b41dd6);
}
impl ::windows::core::RuntimeName for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintDeviceCapabilitiesChangedEventArgs";
}
impl ::core::convert::From<PrintSupportPrintDeviceCapabilitiesChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintSupportPrintDeviceCapabilitiesChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintSupportPrintDeviceCapabilitiesChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportPrintDeviceCapabilitiesChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintSupportPrintDeviceCapabilitiesChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintSupportPrintDeviceCapabilitiesChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintSupportPrintDeviceCapabilitiesChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportPrintDeviceCapabilitiesChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintSupportPrintDeviceCapabilitiesChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
unsafe impl ::core::marker::Sync for PrintSupportPrintDeviceCapabilitiesChangedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintSupportPrintTicketValidationRequestedEventArgs(pub ::windows::core::IInspectable);
impl PrintSupportPrintTicketValidationRequestedEventArgs {
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn PrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PrintTicket::WorkflowPrintTicket>(result__)
        }
    }
    pub fn SetPrintTicketValidationStatus(&self, status: WorkflowPrintTicketValidationStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), status).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintSupportPrintTicketValidationRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs;{338e4e69-db55-55c7-8338-ef64680a8f90})");
}
unsafe impl ::windows::core::Interface for PrintSupportPrintTicketValidationRequestedEventArgs {
    type Vtable = IPrintSupportPrintTicketValidationRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x338e4e69_db55_55c7_8338_ef64680a8f90);
}
impl ::windows::core::RuntimeName for PrintSupportPrintTicketValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportPrintTicketValidationRequestedEventArgs";
}
impl ::core::convert::From<PrintSupportPrintTicketValidationRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintSupportPrintTicketValidationRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintSupportPrintTicketValidationRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportPrintTicketValidationRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintSupportPrintTicketValidationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintSupportPrintTicketValidationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintSupportPrintTicketValidationRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintSupportPrintTicketValidationRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintSupportPrintTicketValidationRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportPrintTicketValidationRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintSupportPrintTicketValidationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintSupportPrintTicketValidationRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintSupportPrintTicketValidationRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PrintSupportPrintTicketValidationRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintSupportSessionInfo(pub ::windows::core::IInspectable);
impl PrintSupportSessionInfo {
    #[cfg(feature = "ApplicationModel")]
    pub fn SourceAppInfo(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::AppInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::AppInfo>(result__)
        }
    }
    #[cfg(feature = "Devices_Printers")]
    pub fn Printer(&self) -> ::windows::core::Result<super::super::super::Devices::Printers::IppPrintDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Devices::Printers::IppPrintDevice>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintSupportSessionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo;{852149af-777d-53e9-9ee9-45d3f4b5be9c})");
}
unsafe impl ::windows::core::Interface for PrintSupportSessionInfo {
    type Vtable = IPrintSupportSessionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x852149af_777d_53e9_9ee9_45d3f4b5be9c);
}
impl ::windows::core::RuntimeName for PrintSupportSessionInfo {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSessionInfo";
}
impl ::core::convert::From<PrintSupportSessionInfo> for ::windows::core::IUnknown {
    fn from(value: PrintSupportSessionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintSupportSessionInfo> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportSessionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintSupportSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintSupportSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintSupportSessionInfo> for ::windows::core::IInspectable {
    fn from(value: PrintSupportSessionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintSupportSessionInfo> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportSessionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintSupportSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintSupportSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintSupportSessionInfo {}
unsafe impl ::core::marker::Sync for PrintSupportSessionInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintSupportSettingsActivatedEventArgs(pub ::windows::core::IInspectable);
impl PrintSupportSettingsActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::super::ApplicationModel::Activation::ActivationKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: super::super::super::ApplicationModel::Activation::ApplicationExecutionState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::User>(result__)
        }
    }
    pub fn Session(&self) -> ::windows::core::Result<PrintSupportSettingsUISession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintSupportSettingsUISession>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintSupportSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs;{1e1b565e-a013-55ea-9b8c-eea39d9fb6c1})");
}
unsafe impl ::windows::core::Interface for PrintSupportSettingsActivatedEventArgs {
    type Vtable = IPrintSupportSettingsActivatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e1b565e_a013_55ea_9b8c_eea39d9fb6c1);
}
impl ::windows::core::RuntimeName for PrintSupportSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsActivatedEventArgs";
}
impl ::core::convert::From<PrintSupportSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PrintSupportSettingsActivatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintSupportSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportSettingsActivatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintSupportSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintSupportSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintSupportSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PrintSupportSettingsActivatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintSupportSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportSettingsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintSupportSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintSupportSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> for PrintSupportSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> for &PrintSupportSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        ::core::convert::TryInto::<super::super::super::ApplicationModel::Activation::IActivatedEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for PrintSupportSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> for &PrintSupportSettingsActivatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
        ::core::convert::TryInto::<super::super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PrintSupportSettingsActivatedEventArgs {}
unsafe impl ::core::marker::Sync for PrintSupportSettingsActivatedEventArgs {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PrintSupportSettingsUISession(pub ::windows::core::IInspectable);
impl PrintSupportSettingsUISession {
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn SessionPrintTicket(&self) -> ::windows::core::Result<super::PrintTicket::WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PrintTicket::WorkflowPrintTicket>(result__)
        }
    }
    pub fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn LaunchKind(&self) -> ::windows::core::Result<SettingsLaunchKind> {
        let this = self;
        unsafe {
            let mut result__: SettingsLaunchKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SettingsLaunchKind>(result__)
        }
    }
    #[cfg(feature = "Graphics_Printing_PrintTicket")]
    pub fn UpdatePrintTicket<'a, Param0: ::windows::core::IntoParam<'a, super::PrintTicket::WorkflowPrintTicket>>(&self, printticket: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), printticket.into_param().abi()).ok() }
    }
    pub fn SessionInfo(&self) -> ::windows::core::Result<PrintSupportSessionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintSupportSessionInfo>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PrintSupportSettingsUISession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession;{c6da2251-83c3-55e4-a0f8-5de8b062adbf})");
}
unsafe impl ::windows::core::Interface for PrintSupportSettingsUISession {
    type Vtable = IPrintSupportSettingsUISession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc6da2251_83c3_55e4_a0f8_5de8b062adbf);
}
impl ::windows::core::RuntimeName for PrintSupportSettingsUISession {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintSupport.PrintSupportSettingsUISession";
}
impl ::core::convert::From<PrintSupportSettingsUISession> for ::windows::core::IUnknown {
    fn from(value: PrintSupportSettingsUISession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PrintSupportSettingsUISession> for ::windows::core::IUnknown {
    fn from(value: &PrintSupportSettingsUISession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintSupportSettingsUISession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintSupportSettingsUISession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PrintSupportSettingsUISession> for ::windows::core::IInspectable {
    fn from(value: PrintSupportSettingsUISession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PrintSupportSettingsUISession> for ::windows::core::IInspectable {
    fn from(value: &PrintSupportSettingsUISession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintSupportSettingsUISession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintSupportSettingsUISession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PrintSupportSettingsUISession {}
unsafe impl ::core::marker::Sync for PrintSupportSettingsUISession {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct SettingsLaunchKind(pub i32);
impl SettingsLaunchKind {
    pub const JobPrintTicket: SettingsLaunchKind = SettingsLaunchKind(0i32);
    pub const UserDefaultPrintTicket: SettingsLaunchKind = SettingsLaunchKind(1i32);
}
impl ::core::convert::From<i32> for SettingsLaunchKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for SettingsLaunchKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SettingsLaunchKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.SettingsLaunchKind;i4)");
}
impl ::windows::core::DefaultType for SettingsLaunchKind {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WorkflowPrintTicketValidationStatus(pub i32);
impl WorkflowPrintTicketValidationStatus {
    pub const Resolved: WorkflowPrintTicketValidationStatus = WorkflowPrintTicketValidationStatus(0i32);
    pub const Conflicting: WorkflowPrintTicketValidationStatus = WorkflowPrintTicketValidationStatus(1i32);
    pub const Invalid: WorkflowPrintTicketValidationStatus = WorkflowPrintTicketValidationStatus(2i32);
}
impl ::core::convert::From<i32> for WorkflowPrintTicketValidationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WorkflowPrintTicketValidationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WorkflowPrintTicketValidationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintSupport.WorkflowPrintTicketValidationStatus;i4)");
}
impl ::windows::core::DefaultType for WorkflowPrintTicketValidationStatus {
    type DefaultType = Self;
}
