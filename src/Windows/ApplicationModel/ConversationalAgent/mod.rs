#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ActivationSignalDetectionConfiguration(pub ::windows::runtime::IInspectable);
impl ActivationSignalDetectionConfiguration {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SignalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn ModelId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn IsActive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SetEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn SetEnabledAsync(&self, value: bool) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn AvailabilityInfo(&self) -> ::windows::runtime::Result<DetectionConfigurationAvailabilityInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DetectionConfigurationAvailabilityInfo>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn AvailabilityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ActivationSignalDetectionConfiguration, DetectionConfigurationAvailabilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn RemoveAvailabilityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Storage_Streams`*"]
    pub fn SetModelData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), datatype.into_param().abi(), data.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`, `Storage_Streams`*"]
    pub fn SetModelDataAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), datatype.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn GetModelDataType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn GetModelDataTypeAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Storage_Streams`*"]
    pub fn GetModelData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`, `Storage_Streams`*"]
    pub fn GetModelDataAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IInputStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IInputStream>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn ClearModelData(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn ClearModelDataAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn TrainingStepsCompleted(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn TrainingStepsRemaining(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn TrainingDataFormat(&self) -> ::windows::runtime::Result<ActivationSignalDetectionTrainingDataFormat> {
        let this = self;
        unsafe {
            let mut result__: ActivationSignalDetectionTrainingDataFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectionTrainingDataFormat>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Storage_Streams`*"]
    pub fn ApplyTrainingData<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: Param1) -> ::windows::runtime::Result<DetectionConfigurationTrainingStatus> {
        let this = self;
        unsafe {
            let mut result__: DetectionConfigurationTrainingStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), trainingdataformat, trainingdata.into_param().abi(), &mut result__).from_abi::<DetectionConfigurationTrainingStatus>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`, `Storage_Streams`*"]
    pub fn ApplyTrainingDataAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), trainingdataformat, trainingdata.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn ClearTrainingData(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn ClearTrainingDataAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Storage_Streams`*"]
    pub fn SetModelDataWithResult<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows::runtime::Result<ActivationSignalDetectionConfigurationSetModelDataResult> {
        let this = &::windows::runtime::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: ActivationSignalDetectionConfigurationSetModelDataResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), datatype.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetectionConfigurationSetModelDataResult>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`, `Storage_Streams`*"]
    pub fn SetModelDataWithResultAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>> {
        let this = &::windows::runtime::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), datatype.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn SetEnabledWithResultAsync(&self, value: bool) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>> {
        let this = &::windows::runtime::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SetEnabledWithResult(&self, value: bool) -> ::windows::runtime::Result<ActivationSignalDetectionConfigurationStateChangeResult> {
        let this = &::windows::runtime::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: ActivationSignalDetectionConfigurationStateChangeResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<ActivationSignalDetectionConfigurationStateChangeResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn TrainingStepCompletionMaxAllowedTime(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ActivationSignalDetectionConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration;{40d8be16-5217-581c-9ab2-ce9b2f2e8e00})");
}
unsafe impl ::windows::runtime::Interface for ActivationSignalDetectionConfiguration {
    type Vtable = IActivationSignalDetectionConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1087946262, 21015, 22556, [154, 178, 206, 155, 47, 46, 142, 0]);
}
impl ::windows::runtime::RuntimeName for ActivationSignalDetectionConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration";
}
impl ::std::convert::From<ActivationSignalDetectionConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: ActivationSignalDetectionConfiguration) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ActivationSignalDetectionConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &ActivationSignalDetectionConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ActivationSignalDetectionConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: ActivationSignalDetectionConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ActivationSignalDetectionConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &ActivationSignalDetectionConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<ActivationSignalDetectionConfiguration> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ActivationSignalDetectionConfiguration) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&ActivationSignalDetectionConfiguration> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ActivationSignalDetectionConfiguration) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ActivationSignalDetectionConfiguration {}
unsafe impl ::std::marker::Sync for ActivationSignalDetectionConfiguration {}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ActivationSignalDetectionConfigurationCreationResult(pub ::windows::runtime::IInspectable);
impl ActivationSignalDetectionConfigurationCreationResult {
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<ActivationSignalDetectionConfigurationCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: ActivationSignalDetectionConfigurationCreationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectionConfigurationCreationStatus>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn Configuration(&self) -> ::windows::runtime::Result<ActivationSignalDetectionConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectionConfiguration>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ActivationSignalDetectionConfigurationCreationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult;{4c89bc1b-8d12-5e48-a71c-7f6bc1cd66e0})");
}
unsafe impl ::windows::runtime::Interface for ActivationSignalDetectionConfigurationCreationResult {
    type Vtable = IActivationSignalDetectionConfigurationCreationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1284095003, 36114, 24136, [167, 28, 127, 107, 193, 205, 102, 224]);
}
impl ::windows::runtime::RuntimeName for ActivationSignalDetectionConfigurationCreationResult {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult";
}
impl ::std::convert::From<ActivationSignalDetectionConfigurationCreationResult> for ::windows::runtime::IUnknown {
    fn from(value: ActivationSignalDetectionConfigurationCreationResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ActivationSignalDetectionConfigurationCreationResult> for ::windows::runtime::IUnknown {
    fn from(value: &ActivationSignalDetectionConfigurationCreationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ActivationSignalDetectionConfigurationCreationResult> for ::windows::runtime::IInspectable {
    fn from(value: ActivationSignalDetectionConfigurationCreationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ActivationSignalDetectionConfigurationCreationResult> for ::windows::runtime::IInspectable {
    fn from(value: &ActivationSignalDetectionConfigurationCreationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ActivationSignalDetectionConfigurationCreationResult {}
unsafe impl ::std::marker::Sync for ActivationSignalDetectionConfigurationCreationResult {}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationCreationStatus(pub i32);
impl ActivationSignalDetectionConfigurationCreationStatus {
    pub const Success: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(0i32);
    pub const SignalIdNotAvailable: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(1i32);
    pub const ModelIdNotSupported: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(2i32);
    pub const InvalidSignalId: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(3i32);
    pub const InvalidModelId: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(4i32);
    pub const InvalidDisplayName: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(5i32);
    pub const ConfigurationAlreadyExists: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(6i32);
    pub const CreationNotSupported: ActivationSignalDetectionConfigurationCreationStatus = ActivationSignalDetectionConfigurationCreationStatus(7i32);
}
impl ::std::convert::From<i32> for ActivationSignalDetectionConfigurationCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ActivationSignalDetectionConfigurationCreationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ActivationSignalDetectionConfigurationCreationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationStatus;i4)");
}
impl ::windows::runtime::DefaultType for ActivationSignalDetectionConfigurationCreationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationRemovalResult(pub i32);
impl ActivationSignalDetectionConfigurationRemovalResult {
    pub const Success: ActivationSignalDetectionConfigurationRemovalResult = ActivationSignalDetectionConfigurationRemovalResult(0i32);
    pub const NotFound: ActivationSignalDetectionConfigurationRemovalResult = ActivationSignalDetectionConfigurationRemovalResult(1i32);
    pub const CurrentlyEnabled: ActivationSignalDetectionConfigurationRemovalResult = ActivationSignalDetectionConfigurationRemovalResult(2i32);
    pub const RemovalNotSupported: ActivationSignalDetectionConfigurationRemovalResult = ActivationSignalDetectionConfigurationRemovalResult(3i32);
}
impl ::std::convert::From<i32> for ActivationSignalDetectionConfigurationRemovalResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ActivationSignalDetectionConfigurationRemovalResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ActivationSignalDetectionConfigurationRemovalResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationRemovalResult;i4)");
}
impl ::windows::runtime::DefaultType for ActivationSignalDetectionConfigurationRemovalResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationSetModelDataResult(pub i32);
impl ActivationSignalDetectionConfigurationSetModelDataResult {
    pub const Success: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(0i32);
    pub const EmptyModelData: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(1i32);
    pub const UnsupportedFormat: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(2i32);
    pub const ConfigurationCurrentlyEnabled: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(3i32);
    pub const InvalidData: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(4i32);
    pub const SetModelDataNotSupported: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(5i32);
    pub const ConfigurationNotFound: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(6i32);
    pub const UnknownError: ActivationSignalDetectionConfigurationSetModelDataResult = ActivationSignalDetectionConfigurationSetModelDataResult(7i32);
}
impl ::std::convert::From<i32> for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ActivationSignalDetectionConfigurationSetModelDataResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ActivationSignalDetectionConfigurationSetModelDataResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationSetModelDataResult;i4)");
}
impl ::windows::runtime::DefaultType for ActivationSignalDetectionConfigurationSetModelDataResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationStateChangeResult(pub i32);
impl ActivationSignalDetectionConfigurationStateChangeResult {
    pub const Success: ActivationSignalDetectionConfigurationStateChangeResult = ActivationSignalDetectionConfigurationStateChangeResult(0i32);
    pub const NoModelData: ActivationSignalDetectionConfigurationStateChangeResult = ActivationSignalDetectionConfigurationStateChangeResult(1i32);
    pub const ConfigurationNotFound: ActivationSignalDetectionConfigurationStateChangeResult = ActivationSignalDetectionConfigurationStateChangeResult(2i32);
}
impl ::std::convert::From<i32> for ActivationSignalDetectionConfigurationStateChangeResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ActivationSignalDetectionConfigurationStateChangeResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ActivationSignalDetectionConfigurationStateChangeResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationStateChangeResult;i4)");
}
impl ::windows::runtime::DefaultType for ActivationSignalDetectionConfigurationStateChangeResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectionTrainingDataFormat(pub i32);
impl ActivationSignalDetectionTrainingDataFormat {
    pub const Voice8kHz8BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(0i32);
    pub const Voice8kHz16BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(1i32);
    pub const Voice16kHz8BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(2i32);
    pub const Voice16kHz16BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(3i32);
    pub const VoiceOEMDefined: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(4i32);
    pub const Audio44kHz8BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(5i32);
    pub const Audio44kHz16BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(6i32);
    pub const Audio48kHz8BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(7i32);
    pub const Audio48kHz16BitMono: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(8i32);
    pub const AudioOEMDefined: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(9i32);
    pub const OtherOEMDefined: ActivationSignalDetectionTrainingDataFormat = ActivationSignalDetectionTrainingDataFormat(10i32);
}
impl ::std::convert::From<i32> for ActivationSignalDetectionTrainingDataFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ActivationSignalDetectionTrainingDataFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ActivationSignalDetectionTrainingDataFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionTrainingDataFormat;i4)");
}
impl ::windows::runtime::DefaultType for ActivationSignalDetectionTrainingDataFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ActivationSignalDetector(pub ::windows::runtime::IInspectable);
impl ActivationSignalDetector {
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn ProviderId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<ActivationSignalDetectorKind> {
        let this = self;
        unsafe {
            let mut result__: ActivationSignalDetectorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectorKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn CanCreateConfigurations(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation_Collections`*"]
    pub fn SupportedModelDataTypes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation_Collections`*"]
    pub fn SupportedTrainingDataFormats(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation_Collections`*"]
    pub fn SupportedPowerStates(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation_Collections`*"]
    pub fn GetSupportedModelIdsForSignalId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), signalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetSupportedModelIdsForSignalIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), signalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn CreateConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn CreateConfigurationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation_Collections`*"]
    pub fn GetConfigurations(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetConfigurationsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn GetConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::runtime::Result<ActivationSignalDetectionConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetectionConfiguration>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn GetConfigurationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn RemoveConfiguration<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn RemoveConfigurationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetAvailableModelIdsForSignalIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>> {
        let this = &::windows::runtime::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), signalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation_Collections`*"]
    pub fn GetAvailableModelIdsForSignalId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), signalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn CreateConfigurationWithResultAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>> {
        let this = &::windows::runtime::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn CreateConfigurationWithResult<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows::runtime::Result<ActivationSignalDetectionConfigurationCreationResult> {
        let this = &::windows::runtime::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetectionConfigurationCreationResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn RemoveConfigurationWithResultAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>> {
        let this = &::windows::runtime::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn RemoveConfigurationWithResult<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::runtime::Result<ActivationSignalDetectionConfigurationRemovalResult> {
        let this = &::windows::runtime::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ActivationSignalDetectionConfigurationRemovalResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetectionConfigurationRemovalResult>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn DetectorId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ActivationSignalDetector {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector;{b5bf345f-a4d0-5b2b-8e65-b3c55ee756ff})");
}
unsafe impl ::windows::runtime::Interface for ActivationSignalDetector {
    type Vtable = IActivationSignalDetector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3049206879, 42192, 23339, [142, 101, 179, 197, 94, 231, 86, 255]);
}
impl ::windows::runtime::RuntimeName for ActivationSignalDetector {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector";
}
impl ::std::convert::From<ActivationSignalDetector> for ::windows::runtime::IUnknown {
    fn from(value: ActivationSignalDetector) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ActivationSignalDetector> for ::windows::runtime::IUnknown {
    fn from(value: &ActivationSignalDetector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ActivationSignalDetector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ActivationSignalDetector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ActivationSignalDetector> for ::windows::runtime::IInspectable {
    fn from(value: ActivationSignalDetector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ActivationSignalDetector> for ::windows::runtime::IInspectable {
    fn from(value: &ActivationSignalDetector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ActivationSignalDetector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ActivationSignalDetector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ActivationSignalDetector {}
unsafe impl ::std::marker::Sync for ActivationSignalDetector {}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectorKind(pub i32);
impl ActivationSignalDetectorKind {
    pub const AudioPattern: ActivationSignalDetectorKind = ActivationSignalDetectorKind(0i32);
    pub const AudioImpulse: ActivationSignalDetectorKind = ActivationSignalDetectorKind(1i32);
    pub const HardwareEvent: ActivationSignalDetectorKind = ActivationSignalDetectorKind(2i32);
}
impl ::std::convert::From<i32> for ActivationSignalDetectorKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ActivationSignalDetectorKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ActivationSignalDetectorKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorKind;i4)");
}
impl ::windows::runtime::DefaultType for ActivationSignalDetectorKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ActivationSignalDetectorPowerState(pub i32);
impl ActivationSignalDetectorPowerState {
    pub const HighPower: ActivationSignalDetectorPowerState = ActivationSignalDetectorPowerState(0i32);
    pub const ConnectedLowPower: ActivationSignalDetectorPowerState = ActivationSignalDetectorPowerState(1i32);
    pub const DisconnectedLowPower: ActivationSignalDetectorPowerState = ActivationSignalDetectorPowerState(2i32);
}
impl ::std::convert::From<i32> for ActivationSignalDetectorPowerState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ActivationSignalDetectorPowerState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ActivationSignalDetectorPowerState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorPowerState;i4)");
}
impl ::windows::runtime::DefaultType for ActivationSignalDetectorPowerState {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConversationalAgentActivationKind(pub i32);
impl ConversationalAgentActivationKind {
    pub const VoiceActivationPreview: ConversationalAgentActivationKind = ConversationalAgentActivationKind(0i32);
    pub const Foreground: ConversationalAgentActivationKind = ConversationalAgentActivationKind(1i32);
}
impl ::std::convert::From<i32> for ConversationalAgentActivationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ConversationalAgentActivationKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentActivationKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationKind;i4)");
}
impl ::windows::runtime::DefaultType for ConversationalAgentActivationKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConversationalAgentActivationResult(pub i32);
impl ConversationalAgentActivationResult {
    pub const Success: ConversationalAgentActivationResult = ConversationalAgentActivationResult(0i32);
    pub const AgentInactive: ConversationalAgentActivationResult = ConversationalAgentActivationResult(1i32);
    pub const ScreenNotAvailable: ConversationalAgentActivationResult = ConversationalAgentActivationResult(2i32);
    pub const AgentInterrupted: ConversationalAgentActivationResult = ConversationalAgentActivationResult(3i32);
}
impl ::std::convert::From<i32> for ConversationalAgentActivationResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ConversationalAgentActivationResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentActivationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationResult;i4)");
}
impl ::windows::runtime::DefaultType for ConversationalAgentActivationResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ConversationalAgentDetectorManager(pub ::windows::runtime::IInspectable);
impl ConversationalAgentDetectorManager {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation_Collections`*"]
    pub fn GetAllActivationSignalDetectors(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetAllActivationSignalDetectorsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation_Collections`*"]
    pub fn GetActivationSignalDetectors(&self, kind: ActivationSignalDetectorKind) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), kind, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetActivationSignalDetectorsAsync(&self, kind: ActivationSignalDetectorKind) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), kind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn Default() -> ::windows::runtime::Result<ConversationalAgentDetectorManager> {
        Self::IConversationalAgentDetectorManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentDetectorManager>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn GetActivationSignalDetectorFromId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, detectorid: Param0) -> ::windows::runtime::Result<ActivationSignalDetector> {
        let this = &::windows::runtime::Interface::cast::<IConversationalAgentDetectorManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), detectorid.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetector>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn GetActivationSignalDetectorFromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, detectorid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetector>> {
        let this = &::windows::runtime::Interface::cast::<IConversationalAgentDetectorManager2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), detectorid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetector>>(result__)
        }
    }
    pub fn IConversationalAgentDetectorManagerStatics<R, F: FnOnce(&IConversationalAgentDetectorManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ConversationalAgentDetectorManager, IConversationalAgentDetectorManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentDetectorManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager;{de94fbb0-597a-5df8-8cfb-9dbb583ba3ff})");
}
unsafe impl ::windows::runtime::Interface for ConversationalAgentDetectorManager {
    type Vtable = IConversationalAgentDetectorManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3734305712, 22906, 24056, [140, 251, 157, 187, 88, 59, 163, 255]);
}
impl ::windows::runtime::RuntimeName for ConversationalAgentDetectorManager {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager";
}
impl ::std::convert::From<ConversationalAgentDetectorManager> for ::windows::runtime::IUnknown {
    fn from(value: ConversationalAgentDetectorManager) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ConversationalAgentDetectorManager> for ::windows::runtime::IUnknown {
    fn from(value: &ConversationalAgentDetectorManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ConversationalAgentDetectorManager> for ::windows::runtime::IInspectable {
    fn from(value: ConversationalAgentDetectorManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ConversationalAgentDetectorManager> for ::windows::runtime::IInspectable {
    fn from(value: &ConversationalAgentDetectorManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ConversationalAgentDetectorManager {}
unsafe impl ::std::marker::Sync for ConversationalAgentDetectorManager {}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ConversationalAgentSession(pub ::windows::runtime::IInspectable);
impl ConversationalAgentSession {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn SessionInterrupted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSessionInterruptedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn RemoveSessionInterrupted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn SignalDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSignalDetectedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn RemoveSignalDetected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn SystemStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSystemStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn RemoveSystemStateChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn AgentState(&self) -> ::windows::runtime::Result<ConversationalAgentState> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentState>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn Signal(&self) -> ::windows::runtime::Result<ConversationalAgentSignal> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentSignal>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn IsIndicatorLightAvailable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn IsScreenAvailable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn IsUserAuthenticated(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn IsVoiceActivationAvailable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn IsInterruptible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn IsInterrupted(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn RequestInterruptibleAsync(&self, interruptible: bool) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), interruptible, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn RequestInterruptible(&self, interruptible: bool) -> ::windows::runtime::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentSessionUpdateResponse = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), interruptible, &mut result__).from_abi::<ConversationalAgentSessionUpdateResponse>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn RequestAgentStateChangeAsync(&self, state: ConversationalAgentState) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), state, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn RequestAgentStateChange(&self, state: ConversationalAgentState) -> ::windows::runtime::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentSessionUpdateResponse = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), state, &mut result__).from_abi::<ConversationalAgentSessionUpdateResponse>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn RequestForegroundActivationAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn RequestForegroundActivation(&self) -> ::windows::runtime::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentSessionUpdateResponse = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentSessionUpdateResponse>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn GetAudioClientAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn GetAudioClient(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Media_Audio"))]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`, `Media_Audio`*"]
    pub fn CreateAudioDeviceInputNodeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Media::Audio::AudioGraph>>(&self, graph: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Media::Audio::AudioDeviceInputNode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), graph.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Media::Audio::AudioDeviceInputNode>>(result__)
        }
    }
    #[cfg(feature = "Media_Audio")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Media_Audio`*"]
    pub fn CreateAudioDeviceInputNode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Media::Audio::AudioGraph>>(&self, graph: Param0) -> ::windows::runtime::Result<super::super::Media::Audio::AudioDeviceInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), graph.into_param().abi(), &mut result__).from_abi::<super::super::Media::Audio::AudioDeviceInputNode>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn GetAudioCaptureDeviceIdAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn GetAudioCaptureDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn GetAudioRenderDeviceIdAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn GetAudioRenderDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn GetSignalModelIdAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn GetSignalModelId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn SetSignalModelIdAsync(&self, signalmodelid: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), signalmodelid, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SetSignalModelId(&self, signalmodelid: u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), signalmodelid, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetSupportedSignalModelIdsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation_Collections`*"]
    pub fn GetSupportedSignalModelIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn GetCurrentSessionAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSession>> {
        Self::IConversationalAgentSessionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentSession>>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn GetCurrentSessionSync() -> ::windows::runtime::Result<ConversationalAgentSession> {
        Self::IConversationalAgentSessionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentSession>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn RequestActivationAsync(&self, activationkind: ConversationalAgentActivationKind) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentActivationResult>> {
        let this = &::windows::runtime::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), activationkind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentActivationResult>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn RequestActivation(&self, activationkind: ConversationalAgentActivationKind) -> ::windows::runtime::Result<ConversationalAgentActivationResult> {
        let this = &::windows::runtime::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ConversationalAgentActivationResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), activationkind, &mut result__).from_abi::<ConversationalAgentActivationResult>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn SetSupportLockScreenActivationAsync(&self, lockscreenactivationsupported: bool) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), lockscreenactivationsupported, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SetSupportLockScreenActivation(&self, lockscreenactivationsupported: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), lockscreenactivationsupported).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation_Collections`*"]
    pub fn GetMissingPrerequisites(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>> {
        let this = &::windows::runtime::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetMissingPrerequisitesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>> {
        let this = &::windows::runtime::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>>(result__)
        }
    }
    pub fn IConversationalAgentSessionStatics<R, F: FnOnce(&IConversationalAgentSessionStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ConversationalAgentSession, IConversationalAgentSessionStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession;{daaae09a-b7ba-57e5-ad13-df520f9b6fa7})");
}
unsafe impl ::windows::runtime::Interface for ConversationalAgentSession {
    type Vtable = IConversationalAgentSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3668631706, 47034, 22501, [173, 19, 223, 82, 15, 155, 111, 167]);
}
impl ::windows::runtime::RuntimeName for ConversationalAgentSession {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession";
}
impl ::std::convert::From<ConversationalAgentSession> for ::windows::runtime::IUnknown {
    fn from(value: ConversationalAgentSession) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ConversationalAgentSession> for ::windows::runtime::IUnknown {
    fn from(value: &ConversationalAgentSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ConversationalAgentSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ConversationalAgentSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ConversationalAgentSession> for ::windows::runtime::IInspectable {
    fn from(value: ConversationalAgentSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ConversationalAgentSession> for ::windows::runtime::IInspectable {
    fn from(value: &ConversationalAgentSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ConversationalAgentSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ConversationalAgentSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<ConversationalAgentSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ConversationalAgentSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&ConversationalAgentSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ConversationalAgentSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for ConversationalAgentSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &ConversationalAgentSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ConversationalAgentSession {}
unsafe impl ::std::marker::Sync for ConversationalAgentSession {}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ConversationalAgentSessionInterruptedEventArgs(pub ::windows::runtime::IInspectable);
impl ConversationalAgentSessionInterruptedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentSessionInterruptedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs;{9766591f-f63d-5d3e-9bf2-bd0760552686})");
}
unsafe impl ::windows::runtime::Interface for ConversationalAgentSessionInterruptedEventArgs {
    type Vtable = IConversationalAgentSessionInterruptedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2540067103, 63037, 23870, [155, 242, 189, 7, 96, 85, 38, 134]);
}
impl ::windows::runtime::RuntimeName for ConversationalAgentSessionInterruptedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs";
}
impl ::std::convert::From<ConversationalAgentSessionInterruptedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ConversationalAgentSessionInterruptedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ConversationalAgentSessionInterruptedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ConversationalAgentSessionInterruptedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ConversationalAgentSessionInterruptedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ConversationalAgentSessionInterruptedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ConversationalAgentSessionInterruptedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ConversationalAgentSessionInterruptedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ConversationalAgentSessionInterruptedEventArgs {}
unsafe impl ::std::marker::Sync for ConversationalAgentSessionInterruptedEventArgs {}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConversationalAgentSessionUpdateResponse(pub i32);
impl ConversationalAgentSessionUpdateResponse {
    pub const Success: ConversationalAgentSessionUpdateResponse = ConversationalAgentSessionUpdateResponse(0i32);
    pub const Failed: ConversationalAgentSessionUpdateResponse = ConversationalAgentSessionUpdateResponse(1i32);
}
impl ::std::convert::From<i32> for ConversationalAgentSessionUpdateResponse {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ConversationalAgentSessionUpdateResponse {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentSessionUpdateResponse {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionUpdateResponse;i4)");
}
impl ::windows::runtime::DefaultType for ConversationalAgentSessionUpdateResponse {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ConversationalAgentSignal(pub ::windows::runtime::IInspectable);
impl ConversationalAgentSignal {
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn IsSignalVerificationRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SetIsSignalVerificationRequired(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SignalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SetSignalId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SignalName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SetSignalName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SignalContext(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SetSignalContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn SignalStart(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn SetSignalStart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn SignalEnd(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation`*"]
    pub fn SetSignalEnd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn DetectorId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IConversationalAgentSignal2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn DetectorKind(&self) -> ::windows::runtime::Result<ActivationSignalDetectorKind> {
        let this = &::windows::runtime::Interface::cast::<IConversationalAgentSignal2>(self)?;
        unsafe {
            let mut result__: ActivationSignalDetectorKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectorKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentSignal {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal;{20ed25f7-b120-51f2-8603-265d6a47f232})");
}
unsafe impl ::windows::runtime::Interface for ConversationalAgentSignal {
    type Vtable = IConversationalAgentSignal_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(552412663, 45344, 20978, [134, 3, 38, 93, 106, 71, 242, 50]);
}
impl ::windows::runtime::RuntimeName for ConversationalAgentSignal {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal";
}
impl ::std::convert::From<ConversationalAgentSignal> for ::windows::runtime::IUnknown {
    fn from(value: ConversationalAgentSignal) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ConversationalAgentSignal> for ::windows::runtime::IUnknown {
    fn from(value: &ConversationalAgentSignal) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ConversationalAgentSignal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ConversationalAgentSignal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ConversationalAgentSignal> for ::windows::runtime::IInspectable {
    fn from(value: ConversationalAgentSignal) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ConversationalAgentSignal> for ::windows::runtime::IInspectable {
    fn from(value: &ConversationalAgentSignal) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ConversationalAgentSignal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ConversationalAgentSignal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ConversationalAgentSignal {}
unsafe impl ::std::marker::Sync for ConversationalAgentSignal {}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ConversationalAgentSignalDetectedEventArgs(pub ::windows::runtime::IInspectable);
impl ConversationalAgentSignalDetectedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentSignalDetectedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs;{4d57eb8f-f88a-599b-91d3-d604876708bc})");
}
unsafe impl ::windows::runtime::Interface for ConversationalAgentSignalDetectedEventArgs {
    type Vtable = IConversationalAgentSignalDetectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1297607567, 63626, 22939, [145, 211, 214, 4, 135, 103, 8, 188]);
}
impl ::windows::runtime::RuntimeName for ConversationalAgentSignalDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs";
}
impl ::std::convert::From<ConversationalAgentSignalDetectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ConversationalAgentSignalDetectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ConversationalAgentSignalDetectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ConversationalAgentSignalDetectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ConversationalAgentSignalDetectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ConversationalAgentSignalDetectedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ConversationalAgentSignalDetectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ConversationalAgentSignalDetectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ConversationalAgentSignalDetectedEventArgs {}
unsafe impl ::std::marker::Sync for ConversationalAgentSignalDetectedEventArgs {}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConversationalAgentState(pub i32);
impl ConversationalAgentState {
    pub const Inactive: ConversationalAgentState = ConversationalAgentState(0i32);
    pub const Detecting: ConversationalAgentState = ConversationalAgentState(1i32);
    pub const Listening: ConversationalAgentState = ConversationalAgentState(2i32);
    pub const Working: ConversationalAgentState = ConversationalAgentState(3i32);
    pub const Speaking: ConversationalAgentState = ConversationalAgentState(4i32);
    pub const ListeningAndSpeaking: ConversationalAgentState = ConversationalAgentState(5i32);
}
impl ::std::convert::From<i32> for ConversationalAgentState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ConversationalAgentState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentState;i4)");
}
impl ::windows::runtime::DefaultType for ConversationalAgentState {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConversationalAgentSystemStateChangeType(pub i32);
impl ConversationalAgentSystemStateChangeType {
    pub const UserAuthentication: ConversationalAgentSystemStateChangeType = ConversationalAgentSystemStateChangeType(0i32);
    pub const ScreenAvailability: ConversationalAgentSystemStateChangeType = ConversationalAgentSystemStateChangeType(1i32);
    pub const IndicatorLightAvailability: ConversationalAgentSystemStateChangeType = ConversationalAgentSystemStateChangeType(2i32);
    pub const VoiceActivationAvailability: ConversationalAgentSystemStateChangeType = ConversationalAgentSystemStateChangeType(3i32);
}
impl ::std::convert::From<i32> for ConversationalAgentSystemStateChangeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ConversationalAgentSystemStateChangeType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentSystemStateChangeType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangeType;i4)");
}
impl ::windows::runtime::DefaultType for ConversationalAgentSystemStateChangeType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ConversationalAgentSystemStateChangedEventArgs(pub ::windows::runtime::IInspectable);
impl ConversationalAgentSystemStateChangedEventArgs {
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn SystemStateChangeType(&self) -> ::windows::runtime::Result<ConversationalAgentSystemStateChangeType> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentSystemStateChangeType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentSystemStateChangeType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentSystemStateChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs;{1c2c6e3e-2785-59a7-8e71-38adeef79928})");
}
unsafe impl ::windows::runtime::Interface for ConversationalAgentSystemStateChangedEventArgs {
    type Vtable = IConversationalAgentSystemStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(472673854, 10117, 22951, [142, 113, 56, 173, 238, 247, 153, 40]);
}
impl ::windows::runtime::RuntimeName for ConversationalAgentSystemStateChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs";
}
impl ::std::convert::From<ConversationalAgentSystemStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ConversationalAgentSystemStateChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&ConversationalAgentSystemStateChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ConversationalAgentSystemStateChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<ConversationalAgentSystemStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ConversationalAgentSystemStateChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ConversationalAgentSystemStateChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ConversationalAgentSystemStateChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ConversationalAgentSystemStateChangedEventArgs {}
unsafe impl ::std::marker::Sync for ConversationalAgentSystemStateChangedEventArgs {}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ConversationalAgentVoiceActivationPrerequisiteKind(pub i32);
impl ConversationalAgentVoiceActivationPrerequisiteKind {
    pub const MicrophonePermission: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(0i32);
    pub const KnownAgents: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(1i32);
    pub const AgentAllowed: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(2i32);
    pub const AppCapability: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(3i32);
    pub const BackgroundTaskRegistration: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(4i32);
    pub const PolicyPermission: ConversationalAgentVoiceActivationPrerequisiteKind = ConversationalAgentVoiceActivationPrerequisiteKind(5i32);
}
impl ::std::convert::From<i32> for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ConversationalAgentVoiceActivationPrerequisiteKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ConversationalAgentVoiceActivationPrerequisiteKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind;i4)");
}
impl ::windows::runtime::DefaultType for ConversationalAgentVoiceActivationPrerequisiteKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityChangeKind(pub i32);
impl DetectionConfigurationAvailabilityChangeKind {
    pub const SystemResourceAccess: DetectionConfigurationAvailabilityChangeKind = DetectionConfigurationAvailabilityChangeKind(0i32);
    pub const Permission: DetectionConfigurationAvailabilityChangeKind = DetectionConfigurationAvailabilityChangeKind(1i32);
    pub const LockScreenPermission: DetectionConfigurationAvailabilityChangeKind = DetectionConfigurationAvailabilityChangeKind(2i32);
}
impl ::std::convert::From<i32> for DetectionConfigurationAvailabilityChangeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DetectionConfigurationAvailabilityChangeKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DetectionConfigurationAvailabilityChangeKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangeKind;i4)");
}
impl ::windows::runtime::DefaultType for DetectionConfigurationAvailabilityChangeKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DetectionConfigurationAvailabilityChangedEventArgs(pub ::windows::runtime::IInspectable);
impl DetectionConfigurationAvailabilityChangedEventArgs {
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<DetectionConfigurationAvailabilityChangeKind> {
        let this = self;
        unsafe {
            let mut result__: DetectionConfigurationAvailabilityChangeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DetectionConfigurationAvailabilityChangeKind>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DetectionConfigurationAvailabilityChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs;{5129c9fb-4be8-5f14-af2b-88d62b1b4462})");
}
unsafe impl ::windows::runtime::Interface for DetectionConfigurationAvailabilityChangedEventArgs {
    type Vtable = IDetectionConfigurationAvailabilityChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1361693179, 19432, 24340, [175, 43, 136, 214, 43, 27, 68, 98]);
}
impl ::windows::runtime::RuntimeName for DetectionConfigurationAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs";
}
impl ::std::convert::From<DetectionConfigurationAvailabilityChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&DetectionConfigurationAvailabilityChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<DetectionConfigurationAvailabilityChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DetectionConfigurationAvailabilityChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DetectionConfigurationAvailabilityChangedEventArgs {}
unsafe impl ::std::marker::Sync for DetectionConfigurationAvailabilityChangedEventArgs {}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DetectionConfigurationAvailabilityInfo(pub ::windows::runtime::IInspectable);
impl DetectionConfigurationAvailabilityInfo {
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn HasSystemResourceAccess(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn HasPermission(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
    pub fn HasLockScreenPermission(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_ConversationalAgent`, `Foundation_Collections`*"]
    pub fn UnavailableSystemResources(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SignalDetectorResourceKind>> {
        let this = &::windows::runtime::Interface::cast::<IDetectionConfigurationAvailabilityInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SignalDetectorResourceKind>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DetectionConfigurationAvailabilityInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo;{b5affeb0-40f0-5398-b838-91979c2c6208})");
}
unsafe impl ::windows::runtime::Interface for DetectionConfigurationAvailabilityInfo {
    type Vtable = IDetectionConfigurationAvailabilityInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3048210096, 16624, 21400, [184, 56, 145, 151, 156, 44, 98, 8]);
}
impl ::windows::runtime::RuntimeName for DetectionConfigurationAvailabilityInfo {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo";
}
impl ::std::convert::From<DetectionConfigurationAvailabilityInfo> for ::windows::runtime::IUnknown {
    fn from(value: DetectionConfigurationAvailabilityInfo) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&DetectionConfigurationAvailabilityInfo> for ::windows::runtime::IUnknown {
    fn from(value: &DetectionConfigurationAvailabilityInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<DetectionConfigurationAvailabilityInfo> for ::windows::runtime::IInspectable {
    fn from(value: DetectionConfigurationAvailabilityInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DetectionConfigurationAvailabilityInfo> for ::windows::runtime::IInspectable {
    fn from(value: &DetectionConfigurationAvailabilityInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DetectionConfigurationAvailabilityInfo {}
unsafe impl ::std::marker::Sync for DetectionConfigurationAvailabilityInfo {}
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DetectionConfigurationTrainingStatus(pub i32);
impl DetectionConfigurationTrainingStatus {
    pub const Success: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(0i32);
    pub const FormatNotSupported: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(1i32);
    pub const VoiceTooQuiet: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(2i32);
    pub const VoiceTooLoud: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(3i32);
    pub const VoiceTooFast: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(4i32);
    pub const VoiceTooSlow: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(5i32);
    pub const VoiceQualityProblem: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(6i32);
    pub const TrainingSystemInternalError: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(7i32);
    pub const TrainingTimedOut: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(8i32);
    pub const ConfigurationNotFound: DetectionConfigurationTrainingStatus = DetectionConfigurationTrainingStatus(9i32);
}
impl ::std::convert::From<i32> for DetectionConfigurationTrainingStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DetectionConfigurationTrainingStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DetectionConfigurationTrainingStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationTrainingStatus;i4)");
}
impl ::windows::runtime::DefaultType for DetectionConfigurationTrainingStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivationSignalDetectionConfiguration {
    type Vtable = IActivationSignalDetectionConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1087946262, 21015, 22556, [154, 178, 206, 155, 47, 46, 142, 0]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datatype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, data: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datatype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ActivationSignalDetectionTrainingDataFormat) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: ::windows::runtime::RawPtr, result__: *mut DetectionConfigurationTrainingStatus) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivationSignalDetectionConfiguration2 {
    type Vtable = IActivationSignalDetectionConfiguration2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1910091810, 22060, 22478, [167, 139, 139, 79, 240, 20, 91, 171]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datatype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, data: ::windows::runtime::RawPtr, result__: *mut ActivationSignalDetectionConfigurationSetModelDataResult) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, datatype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, data: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool, result__: *mut ActivationSignalDetectionConfigurationStateChangeResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfigurationCreationResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivationSignalDetectionConfigurationCreationResult {
    type Vtable = IActivationSignalDetectionConfigurationCreationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1284095003, 36114, 24136, [167, 28, 127, 107, 193, 205, 102, 224]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfigurationCreationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ActivationSignalDetectionConfigurationCreationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivationSignalDetector(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivationSignalDetector {
    type Vtable = IActivationSignalDetector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3049206879, 42192, 23339, [142, 101, 179, 197, 94, 231, 86, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ActivationSignalDetectorKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, modelid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, modelid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, modelid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, modelid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, modelid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, modelid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IActivationSignalDetector2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivationSignalDetector2 {
    type Vtable = IActivationSignalDetector2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3353495818, 47781, 22994, [133, 209, 186, 66, 247, 207, 120, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetector2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, modelid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, modelid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, modelid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, modelid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ActivationSignalDetectionConfigurationRemovalResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConversationalAgentDetectorManager {
    type Vtable = IConversationalAgentDetectorManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3734305712, 22906, 24056, [140, 251, 157, 187, 88, 59, 163, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: ActivationSignalDetectorKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: ActivationSignalDetectorKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConversationalAgentDetectorManager2 {
    type Vtable = IConversationalAgentDetectorManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2220953393, 55283, 21246, [147, 17, 201, 235, 78, 62, 179, 10]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, detectorid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, detectorid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConversationalAgentDetectorManagerStatics {
    type Vtable = IConversationalAgentDetectorManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(917033603, 64014, 22163, [132, 137, 15, 178, 240, 171, 64, 211]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManagerStatics_abi(
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
pub struct IConversationalAgentSession(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConversationalAgentSession {
    type Vtable = IConversationalAgentSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3668631706, 47034, 22501, [173, 19, 223, 82, 15, 155, 111, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ConversationalAgentState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interruptible: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, interruptible: bool, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, state: ConversationalAgentState, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, state: ConversationalAgentState, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Audio"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, graph: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Audio")))] usize,
    #[cfg(feature = "Media_Audio")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, graph: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Audio"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalmodelid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, signalmodelid: u32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSession2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConversationalAgentSession2 {
    type Vtable = IConversationalAgentSession2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2812935161, 44152, 22527, [149, 150, 172, 199, 161, 201, 166, 7]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSession2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activationkind: ConversationalAgentActivationKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activationkind: ConversationalAgentActivationKind, result__: *mut ConversationalAgentActivationResult) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lockscreenactivationsupported: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lockscreenactivationsupported: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSessionInterruptedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConversationalAgentSessionInterruptedEventArgs {
    type Vtable = IConversationalAgentSessionInterruptedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2540067103, 63037, 23870, [155, 242, 189, 7, 96, 85, 38, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSessionInterruptedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSessionStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConversationalAgentSessionStatics {
    type Vtable = IConversationalAgentSessionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2684687982, 59732, 22382, [190, 4, 17, 184, 237, 16, 243, 123]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSessionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSignal(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConversationalAgentSignal {
    type Vtable = IConversationalAgentSignal_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(552412663, 45344, 20978, [134, 3, 38, 93, 106, 71, 242, 50]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignal_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSignal2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConversationalAgentSignal2 {
    type Vtable = IConversationalAgentSignal2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3503061929, 39547, 23604, [136, 14, 182, 20, 108, 144, 78, 203]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignal2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ActivationSignalDetectorKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSignalDetectedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConversationalAgentSignalDetectedEventArgs {
    type Vtable = IConversationalAgentSignalDetectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1297607567, 63626, 22939, [145, 211, 214, 4, 135, 103, 8, 188]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignalDetectedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IConversationalAgentSystemStateChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IConversationalAgentSystemStateChangedEventArgs {
    type Vtable = IConversationalAgentSystemStateChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(472673854, 10117, 22951, [142, 113, 56, 173, 238, 247, 153, 40]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSystemStateChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ConversationalAgentSystemStateChangeType) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDetectionConfigurationAvailabilityChangedEventArgs {
    type Vtable = IDetectionConfigurationAvailabilityChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1361693179, 19432, 24340, [175, 43, 136, 214, 43, 27, 68, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DetectionConfigurationAvailabilityChangeKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDetectionConfigurationAvailabilityInfo {
    type Vtable = IDetectionConfigurationAvailabilityInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3048210096, 16624, 21400, [184, 56, 145, 151, 156, 44, 98, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDetectionConfigurationAvailabilityInfo2 {
    type Vtable = IDetectionConfigurationAvailabilityInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(820012083, 14515, 23627, [132, 195, 98, 182, 230, 133, 178, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc = "*Required features: `ApplicationModel_ConversationalAgent`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SignalDetectorResourceKind(pub i32);
impl SignalDetectorResourceKind {
    pub const ParallelModelSupport: SignalDetectorResourceKind = SignalDetectorResourceKind(0i32);
    pub const ParallelModelSupportForAgent: SignalDetectorResourceKind = SignalDetectorResourceKind(1i32);
    pub const ParallelSignalSupport: SignalDetectorResourceKind = SignalDetectorResourceKind(2i32);
    pub const ParallelSignalSupportForAgent: SignalDetectorResourceKind = SignalDetectorResourceKind(3i32);
    pub const DisplayOffSupport: SignalDetectorResourceKind = SignalDetectorResourceKind(4i32);
    pub const PluggedInPower: SignalDetectorResourceKind = SignalDetectorResourceKind(5i32);
    pub const Detector: SignalDetectorResourceKind = SignalDetectorResourceKind(6i32);
    pub const SupportedSleepState: SignalDetectorResourceKind = SignalDetectorResourceKind(7i32);
    pub const SupportedBatterySaverState: SignalDetectorResourceKind = SignalDetectorResourceKind(8i32);
    pub const ScreenAvailability: SignalDetectorResourceKind = SignalDetectorResourceKind(9i32);
    pub const InputHardware: SignalDetectorResourceKind = SignalDetectorResourceKind(10i32);
    pub const AcousticEchoCancellation: SignalDetectorResourceKind = SignalDetectorResourceKind(11i32);
    pub const ModelIdSupport: SignalDetectorResourceKind = SignalDetectorResourceKind(12i32);
    pub const DataChannel: SignalDetectorResourceKind = SignalDetectorResourceKind(13i32);
}
impl ::std::convert::From<i32> for SignalDetectorResourceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SignalDetectorResourceKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SignalDetectorResourceKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.SignalDetectorResourceKind;i4)");
}
impl ::windows::runtime::DefaultType for SignalDetectorResourceKind {
    type DefaultType = Self;
}
