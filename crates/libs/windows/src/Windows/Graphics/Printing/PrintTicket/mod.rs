#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTicketCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTicketCapabilities {
    type Vtable = IPrintTicketCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c45508b_bbdc_4256_a142_2fd615ecb416);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DocumentBindingFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentCollateFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentDuplexFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentHolePunchFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentInputBinFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentNUpFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentStapleFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub JobPasscodeFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageBorderlessFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageMediaSizeFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageMediaTypeFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageOrientationFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageOutputColorFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageOutputQualityFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageResolutionFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetParameterDefinition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTicketFeature(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTicketFeature {
    type Vtable = IPrintTicketFeature_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7607d6a_59f5_4103_8858_b97710963d39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketFeature_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Options: usize,
    pub GetSelectedOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetSelectedOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SelectionType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintTicketFeatureSelectionType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTicketOption(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTicketOption {
    type Vtable = IPrintTicketOption_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb086cf90_b367_4e4b_bd48_9c78a0bb31ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketOption_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetPropertyNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetPropertyNode: usize,
    #[cfg(feature = "Data_Xml_Dom")]
    pub GetScoredPropertyNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    GetScoredPropertyNode: usize,
    pub GetPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetScoredPropertyValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTicketParameterDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTicketParameterDefinition {
    type Vtable = IPrintTicketParameterDefinition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6bab4e4_2962_4c01_b7f3_9a9294eb8335);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketParameterDefinition_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub DataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintTicketParameterDataType) -> ::windows::core::HRESULT,
    pub UnitType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RangeMin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub RangeMax: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTicketParameterInitializer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTicketParameterInitializer {
    type Vtable = IPrintTicketParameterInitializer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e3335bb_a0a5_48b1_9d5c_07116ddc597a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketParameterInitializer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintTicketValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintTicketValue {
    type Vtable = IPrintTicketValue_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66b30a32_244d_4e22_a98b_bb3cf1f2dd91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintTicketValue_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintTicketValueType) -> ::windows::core::HRESULT,
    pub GetValueAsInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetValueAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWorkflowPrintTicket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWorkflowPrintTicket {
    type Vtable = IWorkflowPrintTicket_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41d52285_35e8_448e_a8c5_e4b6a2cf826c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkflowPrintTicket_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub XmlNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Data_Xml_Dom")]
    pub XmlNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Data_Xml_Dom"))]
    XmlNode: usize,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentBindingFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentCollateFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentDuplexFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentHolePunchFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentInputBinFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentNUpFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DocumentStapleFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub JobPasscodeFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageBorderlessFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageMediaSizeFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageMediaTypeFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageOrientationFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageOutputColorFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageOutputQualityFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PageResolutionFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetFeature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub NotifyXmlChangedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NotifyXmlChangedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ValidateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ValidateAsync: usize,
    pub GetParameterInitializer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetParameterInitializerAsInteger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, integervalue: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetParameterInitializerAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, xmlnamespace: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, stringvalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub MergeAndValidateTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deltashematicket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWorkflowPrintTicketValidationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWorkflowPrintTicketValidationResult {
    type Vtable = IWorkflowPrintTicketValidationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ad1f392_da7b_4a36_bf36_6a99a62e2059);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWorkflowPrintTicketValidationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Validated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketCapabilities(::windows::core::IUnknown);
impl PrintTicketCapabilities {
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XmlNamespace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`, `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XmlNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Data::Xml::Dom::IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentBindingFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentBindingFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentCollateFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentCollateFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentDuplexFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentDuplexFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentHolePunchFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentHolePunchFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentInputBinFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentInputBinFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentNUpFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentNUpFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentStapleFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentStapleFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn JobPasscodeFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).JobPasscodeFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageBorderlessFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageBorderlessFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageMediaSizeFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageMediaSizeFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageMediaTypeFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageMediaTypeFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageOrientationFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageOrientationFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageOutputColorFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageOutputColorFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageOutputQualityFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageOutputQualityFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageResolutionFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageResolutionFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn GetFeature<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, xmlnamespace: Param1) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFeature)(::core::mem::transmute_copy(this), name.into_param().abi(), xmlnamespace.into_param().abi(), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn GetParameterDefinition<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, xmlnamespace: Param1) -> ::windows::core::Result<PrintTicketParameterDefinition> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetParameterDefinition)(::core::mem::transmute_copy(this), name.into_param().abi(), xmlnamespace.into_param().abi(), &mut result__).from_abi::<PrintTicketParameterDefinition>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTicketCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintTicketCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.PrintTicketCapabilities;{8c45508b-bbdc-4256-a142-2fd615ecb416})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTicketCapabilities {
    type Vtable = IPrintTicketCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTicketCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTicketCapabilities {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketCapabilities";
}
impl ::core::convert::From<PrintTicketCapabilities> for ::windows::core::IUnknown {
    fn from(value: PrintTicketCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTicketCapabilities> for ::windows::core::IUnknown {
    fn from(value: &PrintTicketCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTicketCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTicketCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTicketCapabilities> for ::windows::core::IInspectable {
    fn from(value: PrintTicketCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTicketCapabilities> for ::windows::core::IInspectable {
    fn from(value: &PrintTicketCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTicketCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTicketCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintTicketCapabilities {}
unsafe impl ::core::marker::Sync for PrintTicketCapabilities {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketFeature(::windows::core::IUnknown);
impl PrintTicketFeature {
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XmlNamespace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`, `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XmlNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Data::Xml::Dom::IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn GetOption<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, xmlnamespace: Param1) -> ::windows::core::Result<PrintTicketOption> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetOption)(::core::mem::transmute_copy(this), name.into_param().abi(), xmlnamespace.into_param().abi(), &mut result__).from_abi::<PrintTicketOption>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Options(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PrintTicketOption>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Options)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<PrintTicketOption>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn GetSelectedOption(&self) -> ::windows::core::Result<PrintTicketOption> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSelectedOption)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketOption>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn SetSelectedOption<'a, Param0: ::windows::core::IntoParam<'a, PrintTicketOption>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSelectedOption)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn SelectionType(&self) -> ::windows::core::Result<PrintTicketFeatureSelectionType> {
        let this = self;
        unsafe {
            let mut result__: PrintTicketFeatureSelectionType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SelectionType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeatureSelectionType>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTicketFeature {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintTicketFeature {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.PrintTicketFeature;{e7607d6a-59f5-4103-8858-b97710963d39})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTicketFeature {
    type Vtable = IPrintTicketFeature_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTicketFeature as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTicketFeature {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketFeature";
}
impl ::core::convert::From<PrintTicketFeature> for ::windows::core::IUnknown {
    fn from(value: PrintTicketFeature) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTicketFeature> for ::windows::core::IUnknown {
    fn from(value: &PrintTicketFeature) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTicketFeature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTicketFeature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTicketFeature> for ::windows::core::IInspectable {
    fn from(value: PrintTicketFeature) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTicketFeature> for ::windows::core::IInspectable {
    fn from(value: &PrintTicketFeature) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTicketFeature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTicketFeature {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintTicketFeature {}
unsafe impl ::core::marker::Sync for PrintTicketFeature {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PrintTicketFeatureSelectionType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintTicketFeatureSelectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTicketFeatureSelectionType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTicketFeatureSelectionType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTicket.PrintTicketFeatureSelectionType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketOption(::windows::core::IUnknown);
impl PrintTicketOption {
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XmlNamespace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`, `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XmlNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Data::Xml::Dom::IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`, `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetPropertyNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, xmlnamespace: Param1) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPropertyNode)(::core::mem::transmute_copy(this), name.into_param().abi(), xmlnamespace.into_param().abi(), &mut result__).from_abi::<super::super::super::Data::Xml::Dom::IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`, `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn GetScoredPropertyNode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, xmlnamespace: Param1) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScoredPropertyNode)(::core::mem::transmute_copy(this), name.into_param().abi(), xmlnamespace.into_param().abi(), &mut result__).from_abi::<super::super::super::Data::Xml::Dom::IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn GetPropertyValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, xmlnamespace: Param1) -> ::windows::core::Result<PrintTicketValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPropertyValue)(::core::mem::transmute_copy(this), name.into_param().abi(), xmlnamespace.into_param().abi(), &mut result__).from_abi::<PrintTicketValue>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn GetScoredPropertyValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, xmlnamespace: Param1) -> ::windows::core::Result<PrintTicketValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetScoredPropertyValue)(::core::mem::transmute_copy(this), name.into_param().abi(), xmlnamespace.into_param().abi(), &mut result__).from_abi::<PrintTicketValue>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTicketOption {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintTicketOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.PrintTicketOption;{b086cf90-b367-4e4b-bd48-9c78a0bb31ce})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTicketOption {
    type Vtable = IPrintTicketOption_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTicketOption as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTicketOption {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketOption";
}
impl ::core::convert::From<PrintTicketOption> for ::windows::core::IUnknown {
    fn from(value: PrintTicketOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTicketOption> for ::windows::core::IUnknown {
    fn from(value: &PrintTicketOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTicketOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTicketOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTicketOption> for ::windows::core::IInspectable {
    fn from(value: PrintTicketOption) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTicketOption> for ::windows::core::IInspectable {
    fn from(value: &PrintTicketOption) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTicketOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTicketOption {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintTicketOption {}
unsafe impl ::core::marker::Sync for PrintTicketOption {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PrintTicketParameterDataType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintTicketParameterDataType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTicketParameterDataType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTicketParameterDataType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDataType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketParameterDefinition(::windows::core::IUnknown);
impl PrintTicketParameterDefinition {
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XmlNamespace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`, `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XmlNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Data::Xml::Dom::IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DataType(&self) -> ::windows::core::Result<PrintTicketParameterDataType> {
        let this = self;
        unsafe {
            let mut result__: PrintTicketParameterDataType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketParameterDataType>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn UnitType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnitType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn RangeMin(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RangeMin)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn RangeMax(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RangeMax)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTicketParameterDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintTicketParameterDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDefinition;{d6bab4e4-2962-4c01-b7f3-9a9294eb8335})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTicketParameterDefinition {
    type Vtable = IPrintTicketParameterDefinition_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTicketParameterDefinition as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTicketParameterDefinition {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketParameterDefinition";
}
impl ::core::convert::From<PrintTicketParameterDefinition> for ::windows::core::IUnknown {
    fn from(value: PrintTicketParameterDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTicketParameterDefinition> for ::windows::core::IUnknown {
    fn from(value: &PrintTicketParameterDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTicketParameterDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTicketParameterDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTicketParameterDefinition> for ::windows::core::IInspectable {
    fn from(value: PrintTicketParameterDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTicketParameterDefinition> for ::windows::core::IInspectable {
    fn from(value: &PrintTicketParameterDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTicketParameterDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTicketParameterDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintTicketParameterDefinition {}
unsafe impl ::core::marker::Sync for PrintTicketParameterDefinition {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketParameterInitializer(::windows::core::IUnknown);
impl PrintTicketParameterInitializer {
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XmlNamespace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`, `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XmlNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Data::Xml::Dom::IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, PrintTicketValue>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn Value(&self) -> ::windows::core::Result<PrintTicketValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketValue>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTicketParameterInitializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintTicketParameterInitializer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.PrintTicketParameterInitializer;{5e3335bb-a0a5-48b1-9d5c-07116ddc597a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTicketParameterInitializer {
    type Vtable = IPrintTicketParameterInitializer_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTicketParameterInitializer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTicketParameterInitializer {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketParameterInitializer";
}
impl ::core::convert::From<PrintTicketParameterInitializer> for ::windows::core::IUnknown {
    fn from(value: PrintTicketParameterInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTicketParameterInitializer> for ::windows::core::IUnknown {
    fn from(value: &PrintTicketParameterInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTicketParameterInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTicketParameterInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTicketParameterInitializer> for ::windows::core::IInspectable {
    fn from(value: PrintTicketParameterInitializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTicketParameterInitializer> for ::windows::core::IInspectable {
    fn from(value: &PrintTicketParameterInitializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTicketParameterInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTicketParameterInitializer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintTicketParameterInitializer {}
unsafe impl ::core::marker::Sync for PrintTicketParameterInitializer {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct PrintTicketValue(::windows::core::IUnknown);
impl PrintTicketValue {
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<PrintTicketValueType> {
        let this = self;
        unsafe {
            let mut result__: PrintTicketValueType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketValueType>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn GetValueAsInteger(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValueAsInteger)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn GetValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValueAsString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PrintTicketValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PrintTicketValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.PrintTicketValue;{66b30a32-244d-4e22-a98b-bb3cf1f2dd91})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintTicketValue {
    type Vtable = IPrintTicketValue_Vtbl;
    const IID: ::windows::core::GUID = <IPrintTicketValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintTicketValue {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.PrintTicketValue";
}
impl ::core::convert::From<PrintTicketValue> for ::windows::core::IUnknown {
    fn from(value: PrintTicketValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTicketValue> for ::windows::core::IUnknown {
    fn from(value: &PrintTicketValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintTicketValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintTicketValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintTicketValue> for ::windows::core::IInspectable {
    fn from(value: PrintTicketValue) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintTicketValue> for ::windows::core::IInspectable {
    fn from(value: &PrintTicketValue) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintTicketValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintTicketValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PrintTicketValue {}
unsafe impl ::core::marker::Sync for PrintTicketValue {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for PrintTicketValueType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintTicketValueType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintTicketValueType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintTicketValueType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing.PrintTicket.PrintTicketValueType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct WorkflowPrintTicket(::windows::core::IUnknown);
impl WorkflowPrintTicket {
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XmlNamespace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`, `\"Data_Xml_Dom\"`*"]
    #[cfg(feature = "Data_Xml_Dom")]
    pub fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).XmlNode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Data::Xml::Dom::IXmlNode>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn GetCapabilities(&self) -> ::windows::core::Result<PrintTicketCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentBindingFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentBindingFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentCollateFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentCollateFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentDuplexFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentDuplexFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentHolePunchFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentHolePunchFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentInputBinFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentInputBinFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentNUpFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentNUpFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn DocumentStapleFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentStapleFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn JobPasscodeFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).JobPasscodeFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageBorderlessFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageBorderlessFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageMediaSizeFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageMediaSizeFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageMediaTypeFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageMediaTypeFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageOrientationFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageOrientationFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageOutputColorFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageOutputColorFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageOutputQualityFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageOutputQualityFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn PageResolutionFeature(&self) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageResolutionFeature)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn GetFeature<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, xmlnamespace: Param1) -> ::windows::core::Result<PrintTicketFeature> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFeature)(::core::mem::transmute_copy(this), name.into_param().abi(), xmlnamespace.into_param().abi(), &mut result__).from_abi::<PrintTicketFeature>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NotifyXmlChangedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).NotifyXmlChangedAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ValidateAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WorkflowPrintTicketValidationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValidateAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WorkflowPrintTicketValidationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn GetParameterInitializer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, xmlnamespace: Param1) -> ::windows::core::Result<PrintTicketParameterInitializer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetParameterInitializer)(::core::mem::transmute_copy(this), name.into_param().abi(), xmlnamespace.into_param().abi(), &mut result__).from_abi::<PrintTicketParameterInitializer>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn SetParameterInitializerAsInteger<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, xmlnamespace: Param1, integervalue: i32) -> ::windows::core::Result<PrintTicketParameterInitializer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetParameterInitializerAsInteger)(::core::mem::transmute_copy(this), name.into_param().abi(), xmlnamespace.into_param().abi(), integervalue, &mut result__).from_abi::<PrintTicketParameterInitializer>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn SetParameterInitializerAsString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, xmlnamespace: Param1, stringvalue: Param2) -> ::windows::core::Result<PrintTicketParameterInitializer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetParameterInitializerAsString)(::core::mem::transmute_copy(this), name.into_param().abi(), xmlnamespace.into_param().abi(), stringvalue.into_param().abi(), &mut result__).from_abi::<PrintTicketParameterInitializer>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn MergeAndValidateTicket<'a, Param0: ::windows::core::IntoParam<'a, WorkflowPrintTicket>>(&self, deltashematicket: Param0) -> ::windows::core::Result<WorkflowPrintTicket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MergeAndValidateTicket)(::core::mem::transmute_copy(this), deltashematicket.into_param().abi(), &mut result__).from_abi::<WorkflowPrintTicket>(result__)
        }
    }
}
impl ::core::clone::Clone for WorkflowPrintTicket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WorkflowPrintTicket {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket;{41d52285-35e8-448e-a8c5-e4b6a2cf826c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WorkflowPrintTicket {
    type Vtable = IWorkflowPrintTicket_Vtbl;
    const IID: ::windows::core::GUID = <IWorkflowPrintTicket as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WorkflowPrintTicket {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicket";
}
impl ::core::convert::From<WorkflowPrintTicket> for ::windows::core::IUnknown {
    fn from(value: WorkflowPrintTicket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WorkflowPrintTicket> for ::windows::core::IUnknown {
    fn from(value: &WorkflowPrintTicket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WorkflowPrintTicket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WorkflowPrintTicket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WorkflowPrintTicket> for ::windows::core::IInspectable {
    fn from(value: WorkflowPrintTicket) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WorkflowPrintTicket> for ::windows::core::IInspectable {
    fn from(value: &WorkflowPrintTicket) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WorkflowPrintTicket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WorkflowPrintTicket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WorkflowPrintTicket {}
unsafe impl ::core::marker::Sync for WorkflowPrintTicket {}
#[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
pub struct WorkflowPrintTicketValidationResult(::windows::core::IUnknown);
impl WorkflowPrintTicketValidationResult {
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn Validated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Validated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Printing_PrintTicket\"`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedError)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
impl ::core::clone::Clone for WorkflowPrintTicketValidationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for WorkflowPrintTicketValidationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult;{0ad1f392-da7b-4a36-bf36-6a99a62e2059})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WorkflowPrintTicketValidationResult {
    type Vtable = IWorkflowPrintTicketValidationResult_Vtbl;
    const IID: ::windows::core::GUID = <IWorkflowPrintTicketValidationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WorkflowPrintTicketValidationResult {
    const NAME: &'static str = "Windows.Graphics.Printing.PrintTicket.WorkflowPrintTicketValidationResult";
}
impl ::core::convert::From<WorkflowPrintTicketValidationResult> for ::windows::core::IUnknown {
    fn from(value: WorkflowPrintTicketValidationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WorkflowPrintTicketValidationResult> for ::windows::core::IUnknown {
    fn from(value: &WorkflowPrintTicketValidationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WorkflowPrintTicketValidationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WorkflowPrintTicketValidationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WorkflowPrintTicketValidationResult> for ::windows::core::IInspectable {
    fn from(value: WorkflowPrintTicketValidationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WorkflowPrintTicketValidationResult> for ::windows::core::IInspectable {
    fn from(value: &WorkflowPrintTicketValidationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WorkflowPrintTicketValidationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WorkflowPrintTicketValidationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WorkflowPrintTicketValidationResult {}
unsafe impl ::core::marker::Sync for WorkflowPrintTicketValidationResult {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
