#[cfg(feature = "implement_exclusive")]
pub trait IPrintTicketCapabilitiesImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn DocumentBindingFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentCollateFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentDuplexFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentHolePunchFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentInputBinFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentNUpFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentStapleFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn JobPasscodeFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageBorderlessFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageMediaSizeFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageMediaTypeFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOrientationFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOutputColorFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOutputQualityFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageResolutionFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn GetFeature(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketFeature>;
    fn GetParameterDefinition(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketParameterDefinition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTicketFeatureImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetOption(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketOption>;
    fn Options(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PrintTicketOption>>;
    fn GetSelectedOption(&self) -> ::windows::core::Result<PrintTicketOption>;
    fn SetSelectedOption(&self, value: &::core::option::Option<PrintTicketOption>) -> ::windows::core::Result<()>;
    fn SelectionType(&self) -> ::windows::core::Result<PrintTicketFeatureSelectionType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTicketOptionImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetPropertyNode(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn GetScoredPropertyNode(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn GetPropertyValue(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketValue>;
    fn GetScoredPropertyValue(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTicketParameterDefinitionImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn DataType(&self) -> ::windows::core::Result<PrintTicketParameterDataType>;
    fn UnitType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RangeMin(&self) -> ::windows::core::Result<i32>;
    fn RangeMax(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTicketParameterInitializerImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn SetValue(&self, value: &::core::option::Option<PrintTicketValue>) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<PrintTicketValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTicketValueImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<PrintTicketValueType>;
    fn GetValueAsInteger(&self) -> ::windows::core::Result<i32>;
    fn GetValueAsString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWorkflowPrintTicketImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn XmlNode(&self) -> ::windows::core::Result<super::super::super::Data::Xml::Dom::IXmlNode>;
    fn GetCapabilities(&self) -> ::windows::core::Result<PrintTicketCapabilities>;
    fn DocumentBindingFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentCollateFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentDuplexFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentHolePunchFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentInputBinFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentNUpFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn DocumentStapleFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn JobPasscodeFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageBorderlessFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageMediaSizeFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageMediaTypeFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOrientationFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOutputColorFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageOutputQualityFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn PageResolutionFeature(&self) -> ::windows::core::Result<PrintTicketFeature>;
    fn GetFeature(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketFeature>;
    fn NotifyXmlChangedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ValidateAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WorkflowPrintTicketValidationResult>>;
    fn GetParameterInitializer(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketParameterInitializer>;
    fn SetParameterInitializerAsInteger(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING, integervalue: i32) -> ::windows::core::Result<PrintTicketParameterInitializer>;
    fn SetParameterInitializerAsString(&self, name: &::windows::core::HSTRING, xmlnamespace: &::windows::core::HSTRING, stringvalue: &::windows::core::HSTRING) -> ::windows::core::Result<PrintTicketParameterInitializer>;
    fn MergeAndValidateTicket(&self, deltashematicket: &::core::option::Option<WorkflowPrintTicket>) -> ::windows::core::Result<WorkflowPrintTicket>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IWorkflowPrintTicketValidationResultImpl: Sized {
    fn Validated(&self) -> ::windows::core::Result<bool>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
