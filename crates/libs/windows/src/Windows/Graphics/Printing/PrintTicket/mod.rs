#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTicketCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTicketCapabilities {
    type Vtable = IPrintTicketCapabilities_Vtbl;
}
impl ::core::clone::Clone for IPrintTicketCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTicketCapabilities {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c45508b_bbdc_4256_a142_2fd615ecb416);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketCapabilities_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DocumentBindingFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentCollateFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentDuplexFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentHolePunchFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentInputBinFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentNUpFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentStapleFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub JobPasscodeFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageBorderlessFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageMediaSizeFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageMediaTypeFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageOrientationFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageOutputColorFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageOutputQualityFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageResolutionFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, xmlnamespace: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetParameterDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, xmlnamespace: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTicketFeature(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTicketFeature {
    type Vtable = IPrintTicketFeature_Vtbl;
}
impl ::core::clone::Clone for IPrintTicketFeature {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTicketFeature {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7607d6a_59f5_4103_8858_b97710963d39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketFeature_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, xmlnamespace: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Options: usize,
    pub GetSelectedOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSelectedOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintTicketFeatureSelectionType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTicketOption(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTicketOption {
    type Vtable = IPrintTicketOption_Vtbl;
}
impl ::core::clone::Clone for IPrintTicketOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTicketOption {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb086cf90_b367_4e4b_bd48_9c78a0bb31ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketOption_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetPropertyNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, xmlnamespace: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetPropertyNode: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetScoredPropertyNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, xmlnamespace: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetScoredPropertyNode: usize,
    pub GetPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, xmlnamespace: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetScoredPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, xmlnamespace: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTicketParameterDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTicketParameterDefinition {
    type Vtable = IPrintTicketParameterDefinition_Vtbl;
}
impl ::core::clone::Clone for IPrintTicketParameterDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTicketParameterDefinition {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6bab4e4_2962_4c01_b7f3_9a9294eb8335);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketParameterDefinition_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintTicketParameterDataType) -> ::windows::core::HRESULT,
    pub UnitType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RangeMin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub RangeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTicketParameterInitializer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTicketParameterInitializer {
    type Vtable = IPrintTicketParameterInitializer_Vtbl;
}
impl ::core::clone::Clone for IPrintTicketParameterInitializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTicketParameterInitializer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e3335bb_a0a5_48b1_9d5c_07116ddc597a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketParameterInitializer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTicketValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTicketValue {
    type Vtable = IPrintTicketValue_Vtbl;
}
impl ::core::clone::Clone for IPrintTicketValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintTicketValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66b30a32_244d_4e22_a98b_bb3cf1f2dd91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintTicketValueType) -> ::windows::core::HRESULT,
    pub GetValueAsInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetValueAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWorkflowPrintTicket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWorkflowPrintTicket {
    type Vtable = IWorkflowPrintTicket_Vtbl;
}
impl ::core::clone::Clone for IWorkflowPrintTicket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWorkflowPrintTicket {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41d52285_35e8_448e_a8c5_e4b6a2cf826c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkflowPrintTicket_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentBindingFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentCollateFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentDuplexFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentHolePunchFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentInputBinFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentNUpFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DocumentStapleFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub JobPasscodeFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageBorderlessFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageMediaSizeFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageMediaTypeFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageOrientationFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageOutputColorFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageOutputQualityFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageResolutionFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, xmlnamespace: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NotifyXmlChangedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyXmlChangedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ValidateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidateAsync: usize,
    pub GetParameterInitializer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, xmlnamespace: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetParameterInitializerAsInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, xmlnamespace: ::std::mem::MaybeUninit<::windows::core::HSTRING>, integervalue: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetParameterInitializerAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::HSTRING>, xmlnamespace: ::std::mem::MaybeUninit<::windows::core::HSTRING>, stringvalue: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MergeAndValidateTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deltashematicket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWorkflowPrintTicketValidationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWorkflowPrintTicketValidationResult {
    type Vtable = IWorkflowPrintTicketValidationResult_Vtbl;
}
impl ::core::clone::Clone for IWorkflowPrintTicketValidationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWorkflowPrintTicketValidationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ad1f392_da7b_4a36_bf36_6a99a62e2059);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkflowPrintTicketValidationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Validated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketCapabilities(::windows::core::IUnknown);
impl PrintTicketCapabilities {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).XmlNamespace)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Data::Xml::Dom::IXmlNode>();
            (::windows::core::Interface::vtable(this).XmlNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentBindingFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentBindingFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentCollateFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentCollateFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentDuplexFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentDuplexFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentHolePunchFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentHolePunchFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentInputBinFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentInputBinFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentNUpFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentNUpFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentStapleFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentStapleFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn JobPasscodeFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).JobPasscodeFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageBorderlessFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageBorderlessFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageMediaSizeFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageMediaSizeFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageMediaTypeFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageMediaTypeFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageOrientationFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageOrientationFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageOutputColorFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageOutputColorFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageOutputQualityFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageOutputQualityFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageResolutionFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageResolutionFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetFeature(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).GetFeature)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(xmlnamespace), &mut result__).from_abi(result__)
        }
    }
    pub fn GetParameterDefinition(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketParameterDefinition> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketParameterDefinition>();
            (::windows::core::Interface::vtable(this).GetParameterDefinition)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(xmlnamespace), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintTicketCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTicketCapabilities {}
impl ::core::fmt::Debug for PrintTicketCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTicketCapabilities").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintTicketCapabilities {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.PrintTicketCapabilities;{8c45508b-bbdc-4256-a142-2fd615ecb416})");
}
impl ::core::clone::Clone for PrintTicketCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintTicketCapabilities {
    type Vtable = IPrintTicketCapabilities_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintTicketCapabilities {
    const IID: ::windows::core::GUID = <IPrintTicketCapabilities as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintTicketCapabilities {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketCapabilities";
}
::windows::imp::interface_hierarchy!(PrintTicketCapabilities, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintTicketCapabilities {}
unsafe impl ::core::marker::Sync for PrintTicketCapabilities {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketFeature(::windows::core::IUnknown);
impl PrintTicketFeature {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).XmlNamespace)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Data::Xml::Dom::IXmlNode>();
            (::windows::core::Interface::vtable(this).XmlNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetOption(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketOption> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketOption>();
            (::windows::core::Interface::vtable(this).GetOption)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(xmlnamespace), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Options(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PrintTicketOption>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<PrintTicketOption>>();
            (::windows::core::Interface::vtable(this).Options)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetSelectedOption(&self) -> ::windows::core::Result<PrintTicketOption> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketOption>();
            (::windows::core::Interface::vtable(this).GetSelectedOption)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSelectedOption(&self, value: &PrintTicketOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSelectedOption)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SelectionType(&self) -> ::windows::core::Result<PrintTicketFeatureSelectionType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeatureSelectionType>();
            (::windows::core::Interface::vtable(this).SelectionType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintTicketFeature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTicketFeature {}
impl ::core::fmt::Debug for PrintTicketFeature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTicketFeature").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintTicketFeature {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.PrintTicketFeature;{e7607d6a-59f5-4103-8858-b97710963d39})");
}
impl ::core::clone::Clone for PrintTicketFeature {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintTicketFeature {
    type Vtable = IPrintTicketFeature_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintTicketFeature {
    const IID: ::windows::core::GUID = <IPrintTicketFeature as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintTicketFeature {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketFeature";
}
::windows::imp::interface_hierarchy!(PrintTicketFeature, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintTicketFeature {}
unsafe impl ::core::marker::Sync for PrintTicketFeature {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketOption(::windows::core::IUnknown);
impl PrintTicketOption {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).XmlNamespace)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Data::Xml::Dom::IXmlNode>();
            (::windows::core::Interface::vtable(this).XmlNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetPropertyNode(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Data::Xml::Dom::IXmlNode>();
            (::windows::core::Interface::vtable(this).GetPropertyNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(xmlnamespace), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetScoredPropertyNode(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Data::Xml::Dom::IXmlNode>();
            (::windows::core::Interface::vtable(this).GetScoredPropertyNode)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(xmlnamespace), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPropertyValue(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketValue> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketValue>();
            (::windows::core::Interface::vtable(this).GetPropertyValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(xmlnamespace), &mut result__).from_abi(result__)
        }
    }
    pub fn GetScoredPropertyValue(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketValue> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketValue>();
            (::windows::core::Interface::vtable(this).GetScoredPropertyValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(xmlnamespace), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintTicketOption {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTicketOption {}
impl ::core::fmt::Debug for PrintTicketOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTicketOption").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintTicketOption {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.PrintTicketOption;{b086cf90-b367-4e4b-bd48-9c78a0bb31ce})");
}
impl ::core::clone::Clone for PrintTicketOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintTicketOption {
    type Vtable = IPrintTicketOption_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintTicketOption {
    const IID: ::windows::core::GUID = <IPrintTicketOption as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintTicketOption {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketOption";
}
::windows::imp::interface_hierarchy!(PrintTicketOption, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintTicketOption {}
unsafe impl ::core::marker::Sync for PrintTicketOption {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketParameterDefinition(::windows::core::IUnknown);
impl PrintTicketParameterDefinition {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).XmlNamespace)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Data::Xml::Dom::IXmlNode>();
            (::windows::core::Interface::vtable(this).XmlNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DataType(&self) -> ::windows::core::Result<PrintTicketParameterDataType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketParameterDataType>();
            (::windows::core::Interface::vtable(this).DataType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn UnitType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).UnitType)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RangeMin(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).RangeMin)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RangeMax(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).RangeMax)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintTicketParameterDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTicketParameterDefinition {}
impl ::core::fmt::Debug for PrintTicketParameterDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTicketParameterDefinition").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintTicketParameterDefinition {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDefinition;{d6bab4e4-2962-4c01-b7f3-9a9294eb8335})");
}
impl ::core::clone::Clone for PrintTicketParameterDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintTicketParameterDefinition {
    type Vtable = IPrintTicketParameterDefinition_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintTicketParameterDefinition {
    const IID: ::windows::core::GUID = <IPrintTicketParameterDefinition as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintTicketParameterDefinition {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDefinition";
}
::windows::imp::interface_hierarchy!(PrintTicketParameterDefinition, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintTicketParameterDefinition {}
unsafe impl ::core::marker::Sync for PrintTicketParameterDefinition {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketParameterInitializer(::windows::core::IUnknown);
impl PrintTicketParameterInitializer {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).XmlNamespace)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Data::Xml::Dom::IXmlNode>();
            (::windows::core::Interface::vtable(this).XmlNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: &PrintTicketValue) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Value(&self) -> ::windows::core::Result<PrintTicketValue> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketValue>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintTicketParameterInitializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTicketParameterInitializer {}
impl ::core::fmt::Debug for PrintTicketParameterInitializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTicketParameterInitializer").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintTicketParameterInitializer {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.PrintTicketParameterInitializer;{5e3335bb-a0a5-48b1-9d5c-07116ddc597a})");
}
impl ::core::clone::Clone for PrintTicketParameterInitializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintTicketParameterInitializer {
    type Vtable = IPrintTicketParameterInitializer_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintTicketParameterInitializer {
    const IID: ::windows::core::GUID = <IPrintTicketParameterInitializer as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintTicketParameterInitializer {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketParameterInitializer";
}
::windows::imp::interface_hierarchy!(PrintTicketParameterInitializer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintTicketParameterInitializer {}
unsafe impl ::core::marker::Sync for PrintTicketParameterInitializer {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketValue(::windows::core::IUnknown);
impl PrintTicketValue {
    pub fn Type(&self) -> ::windows::core::Result<PrintTicketValueType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketValueType>();
            (::windows::core::Interface::vtable(this).Type)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetValueAsInteger(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).GetValueAsInteger)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).GetValueAsString)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for PrintTicketValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintTicketValue {}
impl ::core::fmt::Debug for PrintTicketValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTicketValue").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintTicketValue {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.PrintTicketValue;{66b30a32-244d-4e22-a98b-bb3cf1f2dd91})");
}
impl ::core::clone::Clone for PrintTicketValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PrintTicketValue {
    type Vtable = IPrintTicketValue_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PrintTicketValue {
    const IID: ::windows::core::GUID = <IPrintTicketValue as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PrintTicketValue {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketValue";
}
::windows::imp::interface_hierarchy!(PrintTicketValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PrintTicketValue {}
unsafe impl ::core::marker::Sync for PrintTicketValue {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct WorkflowPrintTicket(::windows::core::IUnknown);
impl WorkflowPrintTicket {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).XmlNamespace)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Data::Xml::Dom::IXmlNode>();
            (::windows::core::Interface::vtable(this).XmlNode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetCapabilities(&self) -> ::windows::core::Result<PrintTicketCapabilities> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketCapabilities>();
            (::windows::core::Interface::vtable(this).GetCapabilities)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentBindingFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentBindingFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentCollateFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentCollateFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentDuplexFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentDuplexFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentHolePunchFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentHolePunchFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentInputBinFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentInputBinFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentNUpFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentNUpFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DocumentStapleFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).DocumentStapleFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn JobPasscodeFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).JobPasscodeFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageBorderlessFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageBorderlessFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageMediaSizeFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageMediaSizeFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageMediaTypeFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageMediaTypeFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageOrientationFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageOrientationFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageOutputColorFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageOutputColorFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageOutputQualityFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageOutputQualityFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PageResolutionFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).PageResolutionFeature)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetFeature(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketFeature>();
            (::windows::core::Interface::vtable(this).GetFeature)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(xmlnamespace), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyXmlChangedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).NotifyXmlChangedAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValidateAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WorkflowPrintTicketValidationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<WorkflowPrintTicketValidationResult>>();
            (::windows::core::Interface::vtable(this).ValidateAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetParameterInitializer(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketParameterInitializer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketParameterInitializer>();
            (::windows::core::Interface::vtable(this).GetParameterInitializer)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(xmlnamespace), &mut result__).from_abi(result__)
        }
    }
    pub fn SetParameterInitializerAsInteger(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING, integervalue: i32) -> ::windows::core::Result<PrintTicketParameterInitializer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketParameterInitializer>();
            (::windows::core::Interface::vtable(this).SetParameterInitializerAsInteger)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(xmlnamespace), integervalue, &mut result__).from_abi(result__)
        }
    }
    pub fn SetParameterInitializerAsString(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING, stringvalue: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketParameterInitializer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<PrintTicketParameterInitializer>();
            (::windows::core::Interface::vtable(this).SetParameterInitializerAsString)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(xmlnamespace), ::core::mem::transmute_copy(stringvalue), &mut result__).from_abi(result__)
        }
    }
    pub fn MergeAndValidateTicket(&self, deltashematicket: &WorkflowPrintTicket) -> ::windows::core::Result<WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WorkflowPrintTicket>();
            (::windows::core::Interface::vtable(this).MergeAndValidateTicket)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(deltashematicket), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WorkflowPrintTicket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WorkflowPrintTicket {}
impl ::core::fmt::Debug for WorkflowPrintTicket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WorkflowPrintTicket").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WorkflowPrintTicket {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket;{41d52285-35e8-448e-a8c5-e4b6a2cf826c})");
}
impl ::core::clone::Clone for WorkflowPrintTicket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WorkflowPrintTicket {
    type Vtable = IWorkflowPrintTicket_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WorkflowPrintTicket {
    const IID: ::windows::core::GUID = <IWorkflowPrintTicket as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WorkflowPrintTicket {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket";
}
::windows::imp::interface_hierarchy!(WorkflowPrintTicket, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WorkflowPrintTicket {}
unsafe impl ::core::marker::Sync for WorkflowPrintTicket {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct WorkflowPrintTicketValidationResult(::windows::core::IUnknown);
impl WorkflowPrintTicketValidationResult {
    pub fn Validated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Validated)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for WorkflowPrintTicketValidationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WorkflowPrintTicketValidationResult {}
impl ::core::fmt::Debug for WorkflowPrintTicketValidationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WorkflowPrintTicketValidationResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WorkflowPrintTicketValidationResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult;{0ad1f392-da7b-4a36-bf36-6a99a62e2059})");
}
impl ::core::clone::Clone for WorkflowPrintTicketValidationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WorkflowPrintTicketValidationResult {
    type Vtable = IWorkflowPrintTicketValidationResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WorkflowPrintTicketValidationResult {
    const IID: ::windows::core::GUID = <IWorkflowPrintTicketValidationResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WorkflowPrintTicketValidationResult {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult";
}
::windows::imp::interface_hierarchy!(WorkflowPrintTicketValidationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WorkflowPrintTicketValidationResult {}
unsafe impl ::core::marker::Sync for WorkflowPrintTicketValidationResult {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintTicketFeatureSelectionType(pub i32);
impl PrintTicketFeatureSelectionType {
    pub const PickOne: Self = Self(0i32);
    pub const PickMany: Self = Self(1i32);
}
impl ::core::marker::Copy for PrintTicketFeatureSelectionType {}
impl ::core::clone::Clone for PrintTicketFeatureSelectionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintTicketFeatureSelectionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PrintTicketFeatureSelectionType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PrintTicketFeatureSelectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTicketFeatureSelectionType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintTicketFeatureSelectionType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTicket.PrintTicketFeatureSelectionType;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintTicketParameterDataType(pub i32);
impl PrintTicketParameterDataType {
    pub const Integer: Self = Self(0i32);
    pub const NumericString: Self = Self(1i32);
    pub const String: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintTicketParameterDataType {}
impl ::core::clone::Clone for PrintTicketParameterDataType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintTicketParameterDataType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PrintTicketParameterDataType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PrintTicketParameterDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTicketParameterDataType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintTicketParameterDataType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDataType;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintTicketValueType(pub i32);
impl PrintTicketValueType {
    pub const Integer: Self = Self(0i32);
    pub const String: Self = Self(1i32);
    pub const Unknown: Self = Self(2i32);
}
impl ::core::marker::Copy for PrintTicketValueType {}
impl ::core::clone::Clone for PrintTicketValueType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintTicketValueType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PrintTicketValueType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PrintTicketValueType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTicketValueType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for PrintTicketValueType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTicket.PrintTicketValueType;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
