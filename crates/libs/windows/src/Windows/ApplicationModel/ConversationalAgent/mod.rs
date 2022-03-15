#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfiguration(::windows::core::IUnknown);
impl ActivationSignalDetectionConfiguration {
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SignalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn ModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ModelId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn IsActive(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsActive)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEnabledAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetEnabledAsync)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn AvailabilityInfo(&self) -> ::windows::core::Result<DetectionConfigurationAvailabilityInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AvailabilityInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DetectionConfigurationAvailabilityInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AvailabilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ActivationSignalDetectionConfiguration, DetectionConfigurationAvailabilityChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AvailabilityChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAvailabilityChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAvailabilityChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetModelData)(::core::mem::transmute_copy(this), datatype.into_param().abi(), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetModelDataAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetModelDataAsync)(::core::mem::transmute_copy(this), datatype.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn GetModelDataType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetModelDataType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetModelDataTypeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetModelDataTypeAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetModelData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetModelData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetModelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IInputStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetModelDataAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IInputStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn ClearModelData(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearModelData)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearModelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClearModelDataAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn TrainingStepsCompleted(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrainingStepsCompleted)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn TrainingStepsRemaining(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrainingStepsRemaining)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn TrainingDataFormat(&self) -> ::windows::core::Result<ActivationSignalDetectionTrainingDataFormat> {
        let this = self;
        unsafe {
            let mut result__: ActivationSignalDetectionTrainingDataFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrainingDataFormat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectionTrainingDataFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ApplyTrainingData<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: Param1) -> ::windows::core::Result<DetectionConfigurationTrainingStatus> {
        let this = self;
        unsafe {
            let mut result__: DetectionConfigurationTrainingStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplyTrainingData)(::core::mem::transmute_copy(this), trainingdataformat, trainingdata.into_param().abi(), &mut result__).from_abi::<DetectionConfigurationTrainingStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ApplyTrainingDataAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplyTrainingDataAsync)(::core::mem::transmute_copy(this), trainingdataformat, trainingdata.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn ClearTrainingData(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearTrainingData)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClearTrainingDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClearTrainingDataAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelDataWithResult<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows::core::Result<ActivationSignalDetectionConfigurationSetModelDataResult> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: ActivationSignalDetectionConfigurationSetModelDataResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetModelDataWithResult)(::core::mem::transmute_copy(this), datatype.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetectionConfigurationSetModelDataResult>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SetModelDataWithResultAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream>>(&self, datatype: Param0, data: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetModelDataWithResultAsync)(::core::mem::transmute_copy(this), datatype.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEnabledWithResultAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetEnabledWithResultAsync)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SetEnabledWithResult(&self, value: bool) -> ::windows::core::Result<ActivationSignalDetectionConfigurationStateChangeResult> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: ActivationSignalDetectionConfigurationStateChangeResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetEnabledWithResult)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<ActivationSignalDetectionConfigurationStateChangeResult>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn TrainingStepCompletionMaxAllowedTime(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetectionConfiguration2>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrainingStepCompletionMaxAllowedTime)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for ActivationSignalDetectionConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivationSignalDetectionConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivationSignalDetectionConfiguration {}
impl ::core::fmt::Debug for ActivationSignalDetectionConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration;{40d8be16-5217-581c-9ab2-ce9b2f2e8e00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ActivationSignalDetectionConfiguration {
    type Vtable = IActivationSignalDetectionConfiguration_Vtbl;
    const IID: ::windows::core::GUID = <IActivationSignalDetectionConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ActivationSignalDetectionConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfiguration";
}
impl ::core::convert::From<ActivationSignalDetectionConfiguration> for ::windows::core::IUnknown {
    fn from(value: ActivationSignalDetectionConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationSignalDetectionConfiguration> for ::windows::core::IUnknown {
    fn from(value: &ActivationSignalDetectionConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivationSignalDetectionConfiguration> for ::windows::core::IInspectable {
    fn from(value: ActivationSignalDetectionConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationSignalDetectionConfiguration> for ::windows::core::IInspectable {
    fn from(value: &ActivationSignalDetectionConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ActivationSignalDetectionConfiguration> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ActivationSignalDetectionConfiguration) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ActivationSignalDetectionConfiguration> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ActivationSignalDetectionConfiguration) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ActivationSignalDetectionConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ActivationSignalDetectionConfiguration {}
unsafe impl ::core::marker::Sync for ActivationSignalDetectionConfiguration {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetectionConfigurationCreationResult(::windows::core::IUnknown);
impl ActivationSignalDetectionConfigurationCreationResult {
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<ActivationSignalDetectionConfigurationCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: ActivationSignalDetectionConfigurationCreationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectionConfigurationCreationStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn Configuration(&self) -> ::windows::core::Result<ActivationSignalDetectionConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Configuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectionConfiguration>(result__)
        }
    }
}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationCreationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivationSignalDetectionConfigurationCreationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivationSignalDetectionConfigurationCreationResult {}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationCreationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationCreationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationCreationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult;{4c89bc1b-8d12-5e48-a71c-7f6bc1cd66e0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ActivationSignalDetectionConfigurationCreationResult {
    type Vtable = IActivationSignalDetectionConfigurationCreationResult_Vtbl;
    const IID: ::windows::core::GUID = <IActivationSignalDetectionConfigurationCreationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ActivationSignalDetectionConfigurationCreationResult {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationResult";
}
impl ::core::convert::From<ActivationSignalDetectionConfigurationCreationResult> for ::windows::core::IUnknown {
    fn from(value: ActivationSignalDetectionConfigurationCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationSignalDetectionConfigurationCreationResult> for ::windows::core::IUnknown {
    fn from(value: &ActivationSignalDetectionConfigurationCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivationSignalDetectionConfigurationCreationResult> for ::windows::core::IInspectable {
    fn from(value: ActivationSignalDetectionConfigurationCreationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationSignalDetectionConfigurationCreationResult> for ::windows::core::IInspectable {
    fn from(value: &ActivationSignalDetectionConfigurationCreationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ActivationSignalDetectionConfigurationCreationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ActivationSignalDetectionConfigurationCreationResult {}
unsafe impl ::core::marker::Sync for ActivationSignalDetectionConfigurationCreationResult {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectionConfigurationCreationStatus(pub i32);
impl ActivationSignalDetectionConfigurationCreationStatus {
    pub const Success: Self = Self(0i32);
    pub const SignalIdNotAvailable: Self = Self(1i32);
    pub const ModelIdNotSupported: Self = Self(2i32);
    pub const InvalidSignalId: Self = Self(3i32);
    pub const InvalidModelId: Self = Self(4i32);
    pub const InvalidDisplayName: Self = Self(5i32);
    pub const ConfigurationAlreadyExists: Self = Self(6i32);
    pub const CreationNotSupported: Self = Self(7i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationCreationStatus {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationCreationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionConfigurationCreationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionConfigurationCreationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationCreationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationCreationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationCreationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationCreationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectionConfigurationRemovalResult(pub i32);
impl ActivationSignalDetectionConfigurationRemovalResult {
    pub const Success: Self = Self(0i32);
    pub const NotFound: Self = Self(1i32);
    pub const CurrentlyEnabled: Self = Self(2i32);
    pub const RemovalNotSupported: Self = Self(3i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationRemovalResult {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationRemovalResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionConfigurationRemovalResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionConfigurationRemovalResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationRemovalResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationRemovalResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationRemovalResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationRemovalResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectionConfigurationSetModelDataResult(pub i32);
impl ActivationSignalDetectionConfigurationSetModelDataResult {
    pub const Success: Self = Self(0i32);
    pub const EmptyModelData: Self = Self(1i32);
    pub const UnsupportedFormat: Self = Self(2i32);
    pub const ConfigurationCurrentlyEnabled: Self = Self(3i32);
    pub const InvalidData: Self = Self(4i32);
    pub const SetModelDataNotSupported: Self = Self(5i32);
    pub const ConfigurationNotFound: Self = Self(6i32);
    pub const UnknownError: Self = Self(7i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationSetModelDataResult {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionConfigurationSetModelDataResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationSetModelDataResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationSetModelDataResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationSetModelDataResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationSetModelDataResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectionConfigurationStateChangeResult(pub i32);
impl ActivationSignalDetectionConfigurationStateChangeResult {
    pub const Success: Self = Self(0i32);
    pub const NoModelData: Self = Self(1i32);
    pub const ConfigurationNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionConfigurationStateChangeResult {}
impl ::core::clone::Clone for ActivationSignalDetectionConfigurationStateChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionConfigurationStateChangeResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionConfigurationStateChangeResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionConfigurationStateChangeResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionConfigurationStateChangeResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionConfigurationStateChangeResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionConfigurationStateChangeResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectionTrainingDataFormat(pub i32);
impl ActivationSignalDetectionTrainingDataFormat {
    pub const Voice8kHz8BitMono: Self = Self(0i32);
    pub const Voice8kHz16BitMono: Self = Self(1i32);
    pub const Voice16kHz8BitMono: Self = Self(2i32);
    pub const Voice16kHz16BitMono: Self = Self(3i32);
    pub const VoiceOEMDefined: Self = Self(4i32);
    pub const Audio44kHz8BitMono: Self = Self(5i32);
    pub const Audio44kHz16BitMono: Self = Self(6i32);
    pub const Audio48kHz8BitMono: Self = Self(7i32);
    pub const Audio48kHz16BitMono: Self = Self(8i32);
    pub const AudioOEMDefined: Self = Self(9i32);
    pub const OtherOEMDefined: Self = Self(10i32);
}
impl ::core::marker::Copy for ActivationSignalDetectionTrainingDataFormat {}
impl ::core::clone::Clone for ActivationSignalDetectionTrainingDataFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectionTrainingDataFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectionTrainingDataFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectionTrainingDataFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectionTrainingDataFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectionTrainingDataFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectionTrainingDataFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ActivationSignalDetector(::windows::core::IUnknown);
impl ActivationSignalDetector {
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<ActivationSignalDetectorKind> {
        let this = self;
        unsafe {
            let mut result__: ActivationSignalDetectorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectorKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn CanCreateConfigurations(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanCreateConfigurations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedModelDataTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedModelDataTypes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTrainingDataFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedTrainingDataFormats)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedPowerStates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedPowerStates)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedModelIdsForSignalId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedModelIdsForSignalId)(::core::mem::transmute_copy(this), signalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedModelIdsForSignalIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedModelIdsForSignalIdAsync)(::core::mem::transmute_copy(this), signalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn CreateConfiguration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CreateConfiguration)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateConfigurationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateConfigurationAsync)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConfigurations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetConfigurations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetConfigurationsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetConfigurationsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn GetConfiguration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::core::Result<ActivationSignalDetectionConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetConfiguration)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetectionConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetConfigurationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetConfigurationAsync)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn RemoveConfiguration<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveConfiguration)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConfigurationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveConfigurationAsync)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAvailableModelIdsForSignalIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAvailableModelIdsForSignalIdAsync)(::core::mem::transmute_copy(this), signalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAvailableModelIdsForSignalId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAvailableModelIdsForSignalId)(::core::mem::transmute_copy(this), signalid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateConfigurationWithResultAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateConfigurationWithResultAsync)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn CreateConfigurationWithResult<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1, displayname: Param2) -> ::windows::core::Result<ActivationSignalDetectionConfigurationCreationResult> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateConfigurationWithResult)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), displayname.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetectionConfigurationCreationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveConfigurationWithResultAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveConfigurationWithResultAsync)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn RemoveConfigurationWithResult<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, signalid: Param0, modelid: Param1) -> ::windows::core::Result<ActivationSignalDetectionConfigurationRemovalResult> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ActivationSignalDetectionConfigurationRemovalResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveConfigurationWithResult)(::core::mem::transmute_copy(this), signalid.into_param().abi(), modelid.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetectionConfigurationRemovalResult>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn DetectorId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IActivationSignalDetector2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DetectorId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for ActivationSignalDetector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivationSignalDetector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivationSignalDetector {}
impl ::core::fmt::Debug for ActivationSignalDetector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector;{b5bf345f-a4d0-5b2b-8e65-b3c55ee756ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ActivationSignalDetector {
    type Vtable = IActivationSignalDetector_Vtbl;
    const IID: ::windows::core::GUID = <IActivationSignalDetector as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ActivationSignalDetector {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetector";
}
impl ::core::convert::From<ActivationSignalDetector> for ::windows::core::IUnknown {
    fn from(value: ActivationSignalDetector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationSignalDetector> for ::windows::core::IUnknown {
    fn from(value: &ActivationSignalDetector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ActivationSignalDetector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ActivationSignalDetector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ActivationSignalDetector> for ::windows::core::IInspectable {
    fn from(value: ActivationSignalDetector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivationSignalDetector> for ::windows::core::IInspectable {
    fn from(value: &ActivationSignalDetector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ActivationSignalDetector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ActivationSignalDetector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ActivationSignalDetector {}
unsafe impl ::core::marker::Sync for ActivationSignalDetector {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectorKind(pub i32);
impl ActivationSignalDetectorKind {
    pub const AudioPattern: Self = Self(0i32);
    pub const AudioImpulse: Self = Self(1i32);
    pub const HardwareEvent: Self = Self(2i32);
}
impl ::core::marker::Copy for ActivationSignalDetectorKind {}
impl ::core::clone::Clone for ActivationSignalDetectorKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectorKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectorKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectorKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectorKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectorKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ActivationSignalDetectorPowerState(pub i32);
impl ActivationSignalDetectorPowerState {
    pub const HighPower: Self = Self(0i32);
    pub const ConnectedLowPower: Self = Self(1i32);
    pub const DisconnectedLowPower: Self = Self(2i32);
}
impl ::core::marker::Copy for ActivationSignalDetectorPowerState {}
impl ::core::clone::Clone for ActivationSignalDetectorPowerState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ActivationSignalDetectorPowerState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ActivationSignalDetectorPowerState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ActivationSignalDetectorPowerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivationSignalDetectorPowerState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivationSignalDetectorPowerState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ActivationSignalDetectorPowerState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConversationalAgentActivationKind(pub i32);
impl ConversationalAgentActivationKind {
    pub const VoiceActivationPreview: Self = Self(0i32);
    pub const Foreground: Self = Self(1i32);
}
impl ::core::marker::Copy for ConversationalAgentActivationKind {}
impl ::core::clone::Clone for ConversationalAgentActivationKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentActivationKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentActivationKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentActivationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentActivationKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentActivationKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConversationalAgentActivationResult(pub i32);
impl ConversationalAgentActivationResult {
    pub const Success: Self = Self(0i32);
    pub const AgentInactive: Self = Self(1i32);
    pub const ScreenNotAvailable: Self = Self(2i32);
    pub const AgentInterrupted: Self = Self(3i32);
}
impl ::core::marker::Copy for ConversationalAgentActivationResult {}
impl ::core::clone::Clone for ConversationalAgentActivationResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentActivationResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentActivationResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentActivationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentActivationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentActivationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentActivationResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentDetectorManager(::windows::core::IUnknown);
impl ConversationalAgentDetectorManager {
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllActivationSignalDetectors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAllActivationSignalDetectors)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllActivationSignalDetectorsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAllActivationSignalDetectorsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetActivationSignalDetectors(&self, kind: ActivationSignalDetectorKind) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetActivationSignalDetectors)(::core::mem::transmute_copy(this), kind, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetActivationSignalDetectorsAsync(&self, kind: ActivationSignalDetectorKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetActivationSignalDetectorsAsync)(::core::mem::transmute_copy(this), kind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn GetActivationSignalDetectorFromId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, detectorid: Param0) -> ::windows::core::Result<ActivationSignalDetector> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentDetectorManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetActivationSignalDetectorFromId)(::core::mem::transmute_copy(this), detectorid.into_param().abi(), &mut result__).from_abi::<ActivationSignalDetector>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetActivationSignalDetectorFromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, detectorid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetector>> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentDetectorManager2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetActivationSignalDetectorFromIdAsync)(::core::mem::transmute_copy(this), detectorid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ActivationSignalDetector>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn Default() -> ::windows::core::Result<ConversationalAgentDetectorManager> {
        Self::IConversationalAgentDetectorManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Default)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentDetectorManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IConversationalAgentDetectorManagerStatics<R, F: FnOnce(&IConversationalAgentDetectorManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ConversationalAgentDetectorManager, IConversationalAgentDetectorManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ConversationalAgentDetectorManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentDetectorManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentDetectorManager {}
impl ::core::fmt::Debug for ConversationalAgentDetectorManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentDetectorManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentDetectorManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager;{de94fbb0-597a-5df8-8cfb-9dbb583ba3ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ConversationalAgentDetectorManager {
    type Vtable = IConversationalAgentDetectorManager_Vtbl;
    const IID: ::windows::core::GUID = <IConversationalAgentDetectorManager as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentDetectorManager {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentDetectorManager";
}
impl ::core::convert::From<ConversationalAgentDetectorManager> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentDetectorManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentDetectorManager> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentDetectorManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentDetectorManager> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentDetectorManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentDetectorManager> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentDetectorManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConversationalAgentDetectorManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentDetectorManager {}
unsafe impl ::core::marker::Sync for ConversationalAgentDetectorManager {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentSession(::windows::core::IUnknown);
impl ConversationalAgentSession {
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SessionInterrupted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSessionInterruptedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SessionInterrupted)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSessionInterrupted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSessionInterrupted)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignalDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSignalDetectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignalDetected)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSignalDetected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSignalDetected)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSystemStateChangedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemStateChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSystemStateChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSystemStateChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn AgentState(&self) -> ::windows::core::Result<ConversationalAgentState> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AgentState)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn Signal(&self) -> ::windows::core::Result<ConversationalAgentSignal> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Signal)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentSignal>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn IsIndicatorLightAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsIndicatorLightAvailable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn IsScreenAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsScreenAvailable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn IsUserAuthenticated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUserAuthenticated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn IsVoiceActivationAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVoiceActivationAvailable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn IsInterruptible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInterruptible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn IsInterrupted(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInterrupted)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestInterruptibleAsync(&self, interruptible: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestInterruptibleAsync)(::core::mem::transmute_copy(this), interruptible, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn RequestInterruptible(&self, interruptible: bool) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentSessionUpdateResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestInterruptible)(::core::mem::transmute_copy(this), interruptible, &mut result__).from_abi::<ConversationalAgentSessionUpdateResponse>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAgentStateChangeAsync(&self, state: ConversationalAgentState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAgentStateChangeAsync)(::core::mem::transmute_copy(this), state, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn RequestAgentStateChange(&self, state: ConversationalAgentState) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentSessionUpdateResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestAgentStateChange)(::core::mem::transmute_copy(this), state, &mut result__).from_abi::<ConversationalAgentSessionUpdateResponse>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestForegroundActivationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestForegroundActivationAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn RequestForegroundActivation(&self) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentSessionUpdateResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestForegroundActivation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentSessionUpdateResponse>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAudioClientAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAudioClientAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn GetAudioClient(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAudioClient)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`, `\"Media_Audio\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Media_Audio"))]
    pub fn CreateAudioDeviceInputNodeAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Audio::AudioGraph>>(&self, graph: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Media::Audio::AudioDeviceInputNode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAudioDeviceInputNodeAsync)(::core::mem::transmute_copy(this), graph.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Media::Audio::AudioDeviceInputNode>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Media_Audio\"`*"]
    #[cfg(feature = "Media_Audio")]
    pub fn CreateAudioDeviceInputNode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Media::Audio::AudioGraph>>(&self, graph: Param0) -> ::windows::core::Result<super::super::Media::Audio::AudioDeviceInputNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateAudioDeviceInputNode)(::core::mem::transmute_copy(this), graph.into_param().abi(), &mut result__).from_abi::<super::super::Media::Audio::AudioDeviceInputNode>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAudioCaptureDeviceIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAudioCaptureDeviceIdAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn GetAudioCaptureDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAudioCaptureDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetAudioRenderDeviceIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAudioRenderDeviceIdAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn GetAudioRenderDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAudioRenderDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSignalModelIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSignalModelIdAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn GetSignalModelId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSignalModelId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSignalModelIdAsync(&self, signalmodelid: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetSignalModelIdAsync)(::core::mem::transmute_copy(this), signalmodelid, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SetSignalModelId(&self, signalmodelid: u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetSignalModelId)(::core::mem::transmute_copy(this), signalmodelid, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedSignalModelIdsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedSignalModelIdsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedSignalModelIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedSignalModelIds)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestActivationAsync(&self, activationkind: ConversationalAgentActivationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentActivationResult>> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestActivationAsync)(::core::mem::transmute_copy(this), activationkind, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentActivationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn RequestActivation(&self, activationkind: ConversationalAgentActivationKind) -> ::windows::core::Result<ConversationalAgentActivationResult> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ConversationalAgentActivationResult = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestActivation)(::core::mem::transmute_copy(this), activationkind, &mut result__).from_abi::<ConversationalAgentActivationResult>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSupportLockScreenActivationAsync(&self, lockscreenactivationsupported: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetSupportLockScreenActivationAsync)(::core::mem::transmute_copy(this), lockscreenactivationsupported, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SetSupportLockScreenActivation(&self, lockscreenactivationsupported: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSupportLockScreenActivation)(::core::mem::transmute_copy(this), lockscreenactivationsupported).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMissingPrerequisites(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMissingPrerequisites)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMissingPrerequisitesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSession2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMissingPrerequisitesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentSessionAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSession>> {
        Self::IConversationalAgentSessionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentSessionAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ConversationalAgentSession>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn GetCurrentSessionSync() -> ::windows::core::Result<ConversationalAgentSession> {
        Self::IConversationalAgentSessionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentSessionSync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentSession>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IConversationalAgentSessionStatics<R, F: FnOnce(&IConversationalAgentSessionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ConversationalAgentSession, IConversationalAgentSessionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ConversationalAgentSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSession {}
impl ::core::fmt::Debug for ConversationalAgentSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession;{daaae09a-b7ba-57e5-ad13-df520f9b6fa7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ConversationalAgentSession {
    type Vtable = IConversationalAgentSession_Vtbl;
    const IID: ::windows::core::GUID = <IConversationalAgentSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentSession {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSession";
}
impl ::core::convert::From<ConversationalAgentSession> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSession> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConversationalAgentSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentSession> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSession> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConversationalAgentSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ConversationalAgentSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ConversationalAgentSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ConversationalAgentSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ConversationalAgentSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ConversationalAgentSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ConversationalAgentSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSession {}
unsafe impl ::core::marker::Sync for ConversationalAgentSession {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentSessionInterruptedEventArgs(::windows::core::IUnknown);
impl ConversationalAgentSessionInterruptedEventArgs {}
impl ::core::clone::Clone for ConversationalAgentSessionInterruptedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSessionInterruptedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSessionInterruptedEventArgs {}
impl ::core::fmt::Debug for ConversationalAgentSessionInterruptedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSessionInterruptedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSessionInterruptedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs;{9766591f-f63d-5d3e-9bf2-bd0760552686})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ConversationalAgentSessionInterruptedEventArgs {
    type Vtable = IConversationalAgentSessionInterruptedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IConversationalAgentSessionInterruptedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentSessionInterruptedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionInterruptedEventArgs";
}
impl ::core::convert::From<ConversationalAgentSessionInterruptedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentSessionInterruptedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSessionInterruptedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentSessionInterruptedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentSessionInterruptedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentSessionInterruptedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSessionInterruptedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentSessionInterruptedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConversationalAgentSessionInterruptedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSessionInterruptedEventArgs {}
unsafe impl ::core::marker::Sync for ConversationalAgentSessionInterruptedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConversationalAgentSessionUpdateResponse(pub i32);
impl ConversationalAgentSessionUpdateResponse {
    pub const Success: Self = Self(0i32);
    pub const Failed: Self = Self(1i32);
}
impl ::core::marker::Copy for ConversationalAgentSessionUpdateResponse {}
impl ::core::clone::Clone for ConversationalAgentSessionUpdateResponse {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentSessionUpdateResponse {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentSessionUpdateResponse {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentSessionUpdateResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSessionUpdateResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSessionUpdateResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSessionUpdateResponse;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentSignal(::windows::core::IUnknown);
impl ConversationalAgentSignal {
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn IsSignalVerificationRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSignalVerificationRequired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SetIsSignalVerificationRequired(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsSignalVerificationRequired)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SignalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignalId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SetSignalId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSignalId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SignalName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignalName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SetSignalName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSignalName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SignalContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignalContext)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SetSignalContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSignalContext)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignalStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignalStart)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSignalStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSignalStart)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignalEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SignalEnd)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSignalEnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSignalEnd)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn DetectorId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSignal2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DetectorId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn DetectorKind(&self) -> ::windows::core::Result<ActivationSignalDetectorKind> {
        let this = &::windows::core::Interface::cast::<IConversationalAgentSignal2>(self)?;
        unsafe {
            let mut result__: ActivationSignalDetectorKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DetectorKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ActivationSignalDetectorKind>(result__)
        }
    }
}
impl ::core::clone::Clone for ConversationalAgentSignal {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSignal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSignal {}
impl ::core::fmt::Debug for ConversationalAgentSignal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSignal").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSignal {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal;{20ed25f7-b120-51f2-8603-265d6a47f232})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ConversationalAgentSignal {
    type Vtable = IConversationalAgentSignal_Vtbl;
    const IID: ::windows::core::GUID = <IConversationalAgentSignal as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentSignal {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignal";
}
impl ::core::convert::From<ConversationalAgentSignal> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentSignal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSignal> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentSignal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentSignal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConversationalAgentSignal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentSignal> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentSignal) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSignal> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentSignal) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentSignal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConversationalAgentSignal {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSignal {}
unsafe impl ::core::marker::Sync for ConversationalAgentSignal {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentSignalDetectedEventArgs(::windows::core::IUnknown);
impl ConversationalAgentSignalDetectedEventArgs {}
impl ::core::clone::Clone for ConversationalAgentSignalDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSignalDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSignalDetectedEventArgs {}
impl ::core::fmt::Debug for ConversationalAgentSignalDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSignalDetectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSignalDetectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs;{4d57eb8f-f88a-599b-91d3-d604876708bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ConversationalAgentSignalDetectedEventArgs {
    type Vtable = IConversationalAgentSignalDetectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IConversationalAgentSignalDetectedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentSignalDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSignalDetectedEventArgs";
}
impl ::core::convert::From<ConversationalAgentSignalDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentSignalDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSignalDetectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentSignalDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentSignalDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentSignalDetectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSignalDetectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentSignalDetectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConversationalAgentSignalDetectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSignalDetectedEventArgs {}
unsafe impl ::core::marker::Sync for ConversationalAgentSignalDetectedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConversationalAgentState(pub i32);
impl ConversationalAgentState {
    pub const Inactive: Self = Self(0i32);
    pub const Detecting: Self = Self(1i32);
    pub const Listening: Self = Self(2i32);
    pub const Working: Self = Self(3i32);
    pub const Speaking: Self = Self(4i32);
    pub const ListeningAndSpeaking: Self = Self(5i32);
}
impl ::core::marker::Copy for ConversationalAgentState {}
impl ::core::clone::Clone for ConversationalAgentState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentState {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConversationalAgentSystemStateChangeType(pub i32);
impl ConversationalAgentSystemStateChangeType {
    pub const UserAuthentication: Self = Self(0i32);
    pub const ScreenAvailability: Self = Self(1i32);
    pub const IndicatorLightAvailability: Self = Self(2i32);
    pub const VoiceActivationAvailability: Self = Self(3i32);
}
impl ::core::marker::Copy for ConversationalAgentSystemStateChangeType {}
impl ::core::clone::Clone for ConversationalAgentSystemStateChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentSystemStateChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentSystemStateChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentSystemStateChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSystemStateChangeType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSystemStateChangeType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangeType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct ConversationalAgentSystemStateChangedEventArgs(::windows::core::IUnknown);
impl ConversationalAgentSystemStateChangedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn SystemStateChangeType(&self) -> ::windows::core::Result<ConversationalAgentSystemStateChangeType> {
        let this = self;
        unsafe {
            let mut result__: ConversationalAgentSystemStateChangeType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SystemStateChangeType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ConversationalAgentSystemStateChangeType>(result__)
        }
    }
}
impl ::core::clone::Clone for ConversationalAgentSystemStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ConversationalAgentSystemStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ConversationalAgentSystemStateChangedEventArgs {}
impl ::core::fmt::Debug for ConversationalAgentSystemStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentSystemStateChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentSystemStateChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs;{1c2c6e3e-2785-59a7-8e71-38adeef79928})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ConversationalAgentSystemStateChangedEventArgs {
    type Vtable = IConversationalAgentSystemStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IConversationalAgentSystemStateChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ConversationalAgentSystemStateChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.ConversationalAgentSystemStateChangedEventArgs";
}
impl ::core::convert::From<ConversationalAgentSystemStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ConversationalAgentSystemStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSystemStateChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ConversationalAgentSystemStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ConversationalAgentSystemStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ConversationalAgentSystemStateChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ConversationalAgentSystemStateChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ConversationalAgentSystemStateChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ConversationalAgentSystemStateChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ConversationalAgentSystemStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for ConversationalAgentSystemStateChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ConversationalAgentVoiceActivationPrerequisiteKind(pub i32);
impl ConversationalAgentVoiceActivationPrerequisiteKind {
    pub const MicrophonePermission: Self = Self(0i32);
    pub const KnownAgents: Self = Self(1i32);
    pub const AgentAllowed: Self = Self(2i32);
    pub const AppCapability: Self = Self(3i32);
    pub const BackgroundTaskRegistration: Self = Self(4i32);
    pub const PolicyPermission: Self = Self(5i32);
}
impl ::core::marker::Copy for ConversationalAgentVoiceActivationPrerequisiteKind {}
impl ::core::clone::Clone for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ConversationalAgentVoiceActivationPrerequisiteKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for ConversationalAgentVoiceActivationPrerequisiteKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ConversationalAgentVoiceActivationPrerequisiteKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ConversationalAgentVoiceActivationPrerequisiteKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.ConversationalAgentVoiceActivationPrerequisiteKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DetectionConfigurationAvailabilityChangeKind(pub i32);
impl DetectionConfigurationAvailabilityChangeKind {
    pub const SystemResourceAccess: Self = Self(0i32);
    pub const Permission: Self = Self(1i32);
    pub const LockScreenPermission: Self = Self(2i32);
}
impl ::core::marker::Copy for DetectionConfigurationAvailabilityChangeKind {}
impl ::core::clone::Clone for DetectionConfigurationAvailabilityChangeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DetectionConfigurationAvailabilityChangeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DetectionConfigurationAvailabilityChangeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DetectionConfigurationAvailabilityChangeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectionConfigurationAvailabilityChangeKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DetectionConfigurationAvailabilityChangeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangeKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityChangedEventArgs(::windows::core::IUnknown);
impl DetectionConfigurationAvailabilityChangedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn Kind(&self) -> ::windows::core::Result<DetectionConfigurationAvailabilityChangeKind> {
        let this = self;
        unsafe {
            let mut result__: DetectionConfigurationAvailabilityChangeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DetectionConfigurationAvailabilityChangeKind>(result__)
        }
    }
}
impl ::core::clone::Clone for DetectionConfigurationAvailabilityChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DetectionConfigurationAvailabilityChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DetectionConfigurationAvailabilityChangedEventArgs {}
impl ::core::fmt::Debug for DetectionConfigurationAvailabilityChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectionConfigurationAvailabilityChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DetectionConfigurationAvailabilityChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs;{5129c9fb-4be8-5f14-af2b-88d62b1b4462})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DetectionConfigurationAvailabilityChangedEventArgs {
    type Vtable = IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IDetectionConfigurationAvailabilityChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DetectionConfigurationAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityChangedEventArgs";
}
impl ::core::convert::From<DetectionConfigurationAvailabilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DetectionConfigurationAvailabilityChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DetectionConfigurationAvailabilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DetectionConfigurationAvailabilityChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &DetectionConfigurationAvailabilityChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DetectionConfigurationAvailabilityChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DetectionConfigurationAvailabilityChangedEventArgs {}
unsafe impl ::core::marker::Sync for DetectionConfigurationAvailabilityChangedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
pub struct DetectionConfigurationAvailabilityInfo(::windows::core::IUnknown);
impl DetectionConfigurationAvailabilityInfo {
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn HasSystemResourceAccess(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasSystemResourceAccess)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn HasPermission(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasPermission)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
    pub fn HasLockScreenPermission(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasLockScreenPermission)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UnavailableSystemResources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SignalDetectorResourceKind>> {
        let this = &::windows::core::Interface::cast::<IDetectionConfigurationAvailabilityInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnavailableSystemResources)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SignalDetectorResourceKind>>(result__)
        }
    }
}
impl ::core::clone::Clone for DetectionConfigurationAvailabilityInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DetectionConfigurationAvailabilityInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DetectionConfigurationAvailabilityInfo {}
impl ::core::fmt::Debug for DetectionConfigurationAvailabilityInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectionConfigurationAvailabilityInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DetectionConfigurationAvailabilityInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo;{b5affeb0-40f0-5398-b838-91979c2c6208})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DetectionConfigurationAvailabilityInfo {
    type Vtable = IDetectionConfigurationAvailabilityInfo_Vtbl;
    const IID: ::windows::core::GUID = <IDetectionConfigurationAvailabilityInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DetectionConfigurationAvailabilityInfo {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationAvailabilityInfo";
}
impl ::core::convert::From<DetectionConfigurationAvailabilityInfo> for ::windows::core::IUnknown {
    fn from(value: DetectionConfigurationAvailabilityInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DetectionConfigurationAvailabilityInfo> for ::windows::core::IUnknown {
    fn from(value: &DetectionConfigurationAvailabilityInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DetectionConfigurationAvailabilityInfo> for ::windows::core::IInspectable {
    fn from(value: DetectionConfigurationAvailabilityInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DetectionConfigurationAvailabilityInfo> for ::windows::core::IInspectable {
    fn from(value: &DetectionConfigurationAvailabilityInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DetectionConfigurationAvailabilityInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DetectionConfigurationAvailabilityInfo {}
unsafe impl ::core::marker::Sync for DetectionConfigurationAvailabilityInfo {}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DetectionConfigurationTrainingStatus(pub i32);
impl DetectionConfigurationTrainingStatus {
    pub const Success: Self = Self(0i32);
    pub const FormatNotSupported: Self = Self(1i32);
    pub const VoiceTooQuiet: Self = Self(2i32);
    pub const VoiceTooLoud: Self = Self(3i32);
    pub const VoiceTooFast: Self = Self(4i32);
    pub const VoiceTooSlow: Self = Self(5i32);
    pub const VoiceQualityProblem: Self = Self(6i32);
    pub const TrainingSystemInternalError: Self = Self(7i32);
    pub const TrainingTimedOut: Self = Self(8i32);
    pub const ConfigurationNotFound: Self = Self(9i32);
}
impl ::core::marker::Copy for DetectionConfigurationTrainingStatus {}
impl ::core::clone::Clone for DetectionConfigurationTrainingStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DetectionConfigurationTrainingStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DetectionConfigurationTrainingStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DetectionConfigurationTrainingStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DetectionConfigurationTrainingStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DetectionConfigurationTrainingStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.DetectionConfigurationTrainingStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetectionConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivationSignalDetectionConfiguration {
    type Vtable = IActivationSignalDetectionConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40d8be16_5217_581c_9ab2_ce9b2f2e8e00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsActive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetEnabledAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEnabledAsync: usize,
    pub AvailabilityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AvailabilityChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAvailabilityChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAvailabilityChanged: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelData: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetModelDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetModelDataAsync: usize,
    pub GetModelDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetModelDataTypeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetModelDataTypeAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetModelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetModelData: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetModelDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetModelDataAsync: usize,
    pub ClearModelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearModelDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearModelDataAsync: usize,
    pub TrainingStepsCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TrainingStepsRemaining: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TrainingDataFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionTrainingDataFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ApplyTrainingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: ::windows::core::RawPtr, result__: *mut DetectionConfigurationTrainingStatus) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ApplyTrainingData: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub ApplyTrainingDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    ApplyTrainingDataAsync: usize,
    pub ClearTrainingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClearTrainingDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClearTrainingDataAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetectionConfiguration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivationSignalDetectionConfiguration2 {
    type Vtable = IActivationSignalDetectionConfiguration2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71d9b022_562c_57ce_a78b_8b4ff0145bab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfiguration2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelDataWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ActivationSignalDetectionConfigurationSetModelDataResult) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelDataWithResult: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SetModelDataWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SetModelDataWithResultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetEnabledWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEnabledWithResultAsync: usize,
    pub SetEnabledWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ActivationSignalDetectionConfigurationStateChangeResult) -> ::windows::core::HRESULT,
    pub TrainingStepCompletionMaxAllowedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetectionConfigurationCreationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivationSignalDetectionConfigurationCreationResult {
    type Vtable = IActivationSignalDetectionConfigurationCreationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4c89bc1b_8d12_5e48_a71c_7f6bc1cd66e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetectionConfigurationCreationResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionConfigurationCreationStatus) -> ::windows::core::HRESULT,
    pub Configuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetector(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivationSignalDetector {
    type Vtable = IActivationSignalDetector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5bf345f_a4d0_5b2b_8e65_b3c55ee756ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetector_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectorKind) -> ::windows::core::HRESULT,
    pub CanCreateConfigurations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedModelDataTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedModelDataTypes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTrainingDataFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTrainingDataFormats: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedPowerStates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedPowerStates: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedModelIdsForSignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedModelIdsForSignalId: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedModelIdsForSignalIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedModelIdsForSignalIdAsync: usize,
    pub CreateConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateConfigurationAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConfigurations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConfigurations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetConfigurationsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetConfigurationsAsync: usize,
    pub GetConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetConfigurationAsync: usize,
    pub RemoveConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoveConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConfigurationAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivationSignalDetector2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivationSignalDetector2 {
    type Vtable = IActivationSignalDetector2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc7e2490a_baa5_59d2_85d1_ba42f7cf78c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationSignalDetector2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAvailableModelIdsForSignalIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAvailableModelIdsForSignalIdAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAvailableModelIdsForSignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAvailableModelIdsForSignalId: usize,
    #[cfg(feature = "Foundation")]
    pub CreateConfigurationWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateConfigurationWithResultAsync: usize,
    pub CreateConfigurationWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RemoveConfigurationWithResultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveConfigurationWithResultAsync: usize,
    pub RemoveConfigurationWithResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ActivationSignalDetectionConfigurationRemovalResult) -> ::windows::core::HRESULT,
    pub DetectorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentDetectorManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConversationalAgentDetectorManager {
    type Vtable = IConversationalAgentDetectorManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde94fbb0_597a_5df8_8cfb_9dbb583ba3ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllActivationSignalDetectors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllActivationSignalDetectors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllActivationSignalDetectorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllActivationSignalDetectorsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActivationSignalDetectors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: ActivationSignalDetectorKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActivationSignalDetectors: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetActivationSignalDetectorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: ActivationSignalDetectorKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetActivationSignalDetectorsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentDetectorManager2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConversationalAgentDetectorManager2 {
    type Vtable = IConversationalAgentDetectorManager2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84610f31_d7f3_52fe_9311_c9eb4e3eb30a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManager2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetActivationSignalDetectorFromId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, detectorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetActivationSignalDetectorFromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, detectorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetActivationSignalDetectorFromIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentDetectorManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConversationalAgentDetectorManagerStatics {
    type Vtable = IConversationalAgentDetectorManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36a8d283_fa0e_5693_8489_0fb2f0ab40d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentDetectorManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConversationalAgentSession {
    type Vtable = IConversationalAgentSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaaae09a_b7ba_57e5_ad13_df520f9b6fa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SessionInterrupted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SessionInterrupted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSessionInterrupted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSessionInterrupted: usize,
    #[cfg(feature = "Foundation")]
    pub SignalDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSignalDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSignalDetected: usize,
    #[cfg(feature = "Foundation")]
    pub SystemStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SystemStateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSystemStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSystemStateChanged: usize,
    pub AgentState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentState) -> ::windows::core::HRESULT,
    pub Signal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsIndicatorLightAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsScreenAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsUserAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsVoiceActivationAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsInterruptible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsInterrupted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestInterruptibleAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interruptible: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestInterruptibleAsync: usize,
    pub RequestInterruptible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interruptible: bool, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestAgentStateChangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: ConversationalAgentState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAgentStateChangeAsync: usize,
    pub RequestAgentStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: ConversationalAgentState, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestForegroundActivationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestForegroundActivationAsync: usize,
    pub RequestForegroundActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetAudioClientAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAudioClientAsync: usize,
    pub GetAudioClient: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Media_Audio"))]
    pub CreateAudioDeviceInputNodeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, graph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Media_Audio")))]
    CreateAudioDeviceInputNodeAsync: usize,
    #[cfg(feature = "Media_Audio")]
    pub CreateAudioDeviceInputNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, graph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_Audio"))]
    CreateAudioDeviceInputNode: usize,
    #[cfg(feature = "Foundation")]
    pub GetAudioCaptureDeviceIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAudioCaptureDeviceIdAsync: usize,
    pub GetAudioCaptureDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetAudioRenderDeviceIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetAudioRenderDeviceIdAsync: usize,
    pub GetAudioRenderDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetSignalModelIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSignalModelIdAsync: usize,
    pub GetSignalModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetSignalModelIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalmodelid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSignalModelIdAsync: usize,
    pub SetSignalModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalmodelid: u32, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSignalModelIdsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSignalModelIdsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSignalModelIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSignalModelIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSession2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConversationalAgentSession2 {
    type Vtable = IConversationalAgentSession2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7a9fbf9_ac78_57ff_9596_acc7a1c9a607);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSession2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RequestActivationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationkind: ConversationalAgentActivationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestActivationAsync: usize,
    pub RequestActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activationkind: ConversationalAgentActivationKind, result__: *mut ConversationalAgentActivationResult) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetSupportLockScreenActivationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lockscreenactivationsupported: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSupportLockScreenActivationAsync: usize,
    pub SetSupportLockScreenActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lockscreenactivationsupported: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMissingPrerequisites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMissingPrerequisites: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetMissingPrerequisitesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetMissingPrerequisitesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSessionInterruptedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConversationalAgentSessionInterruptedEventArgs {
    type Vtable = IConversationalAgentSessionInterruptedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9766591f_f63d_5d3e_9bf2_bd0760552686);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSessionInterruptedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSessionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConversationalAgentSessionStatics {
    type Vtable = IConversationalAgentSessionStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa005166e_e954_576e_be04_11b8ed10f37b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSessionStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetCurrentSessionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentSessionAsync: usize,
    pub GetCurrentSessionSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSignal(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConversationalAgentSignal {
    type Vtable = IConversationalAgentSignal_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20ed25f7_b120_51f2_8603_265d6a47f232);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignal_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSignalVerificationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSignalVerificationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub SignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSignalId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SignalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSignalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SignalContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSignalContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SignalStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalStart: usize,
    #[cfg(feature = "Foundation")]
    pub SetSignalStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSignalStart: usize,
    #[cfg(feature = "Foundation")]
    pub SignalEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignalEnd: usize,
    #[cfg(feature = "Foundation")]
    pub SetSignalEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSignalEnd: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSignal2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConversationalAgentSignal2 {
    type Vtable = IConversationalAgentSignal2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0cc7ba9_9a7b_5c34_880e_b6146c904ecb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignal2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DetectorId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DetectorKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectorKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSignalDetectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConversationalAgentSignalDetectedEventArgs {
    type Vtable = IConversationalAgentSignalDetectedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d57eb8f_f88a_599b_91d3_d604876708bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSignalDetectedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IConversationalAgentSystemStateChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IConversationalAgentSystemStateChangedEventArgs {
    type Vtable = IConversationalAgentSystemStateChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c2c6e3e_2785_59a7_8e71_38adeef79928);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConversationalAgentSystemStateChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SystemStateChangeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentSystemStateChangeType) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDetectionConfigurationAvailabilityChangedEventArgs {
    type Vtable = IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5129c9fb_4be8_5f14_af2b_88d62b1b4462);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DetectionConfigurationAvailabilityChangeKind) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDetectionConfigurationAvailabilityInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDetectionConfigurationAvailabilityInfo {
    type Vtable = IDetectionConfigurationAvailabilityInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5affeb0_40f0_5398_b838_91979c2c6208);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasSystemResourceAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HasLockScreenPermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDetectionConfigurationAvailabilityInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDetectionConfigurationAvailabilityInfo2 {
    type Vtable = IDetectionConfigurationAvailabilityInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30e06433_38b3_5c4b_84c3_62b6e685b2ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDetectionConfigurationAvailabilityInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub UnavailableSystemResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UnavailableSystemResources: usize,
}
#[doc = "*Required features: `\"ApplicationModel_ConversationalAgent\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SignalDetectorResourceKind(pub i32);
impl SignalDetectorResourceKind {
    pub const ParallelModelSupport: Self = Self(0i32);
    pub const ParallelModelSupportForAgent: Self = Self(1i32);
    pub const ParallelSignalSupport: Self = Self(2i32);
    pub const ParallelSignalSupportForAgent: Self = Self(3i32);
    pub const DisplayOffSupport: Self = Self(4i32);
    pub const PluggedInPower: Self = Self(5i32);
    pub const Detector: Self = Self(6i32);
    pub const SupportedSleepState: Self = Self(7i32);
    pub const SupportedBatterySaverState: Self = Self(8i32);
    pub const ScreenAvailability: Self = Self(9i32);
    pub const InputHardware: Self = Self(10i32);
    pub const AcousticEchoCancellation: Self = Self(11i32);
    pub const ModelIdSupport: Self = Self(12i32);
    pub const DataChannel: Self = Self(13i32);
}
impl ::core::marker::Copy for SignalDetectorResourceKind {}
impl ::core::clone::Clone for SignalDetectorResourceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SignalDetectorResourceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SignalDetectorResourceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for SignalDetectorResourceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignalDetectorResourceKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SignalDetectorResourceKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ConversationalAgent.SignalDetectorResourceKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
