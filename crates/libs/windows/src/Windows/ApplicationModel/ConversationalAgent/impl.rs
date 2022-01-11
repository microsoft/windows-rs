#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IActivationSignalDetectionConfigurationImpl: Sized {
    fn SignalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetEnabledAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn AvailabilityInfo(&self) -> ::windows::core::Result<DetectionConfigurationAvailabilityInfo>;
    fn AvailabilityChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ActivationSignalDetectionConfiguration, DetectionConfigurationAvailabilityChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailabilityChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetModelData(&self, datatype: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn SetModelDataAsync(&self, datatype: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetModelDataType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetModelDataTypeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetModelData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn GetModelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IInputStream>>;
    fn ClearModelData(&self) -> ::windows::core::Result<()>;
    fn ClearModelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TrainingStepsCompleted(&self) -> ::windows::core::Result<u32>;
    fn TrainingStepsRemaining(&self) -> ::windows::core::Result<u32>;
    fn TrainingDataFormat(&self) -> ::windows::core::Result<ActivationSignalDetectionTrainingDataFormat>;
    fn ApplyTrainingData(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<DetectionConfigurationTrainingStatus>;
    fn ApplyTrainingDataAsync(&self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>>;
    fn ClearTrainingData(&self) -> ::windows::core::Result<()>;
    fn ClearTrainingDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivationSignalDetectionConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IActivationSignalDetectionConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationSignalDetectionConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationSignalDetectionConfigurationVtbl {
        unsafe extern "system" fn SignalId<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelId<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsActive<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn SetEnabledAsync<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabledAsync(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailabilityInfo<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailabilityInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailabilityChanged<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailabilityChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ActivationSignalDetectionConfiguration, DetectionConfigurationAvailabilityChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ActivationSignalDetectionConfiguration, DetectionConfigurationAvailabilityChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAvailabilityChanged<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAvailabilityChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetModelData<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModelData(&*(&datatype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetModelDataAsync<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetModelDataAsync(&*(&datatype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModelDataType<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModelDataType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModelDataTypeAsync<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModelDataTypeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModelData<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModelData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModelDataAsync<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModelDataAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearModelData<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearModelData().into()
        }
        unsafe extern "system" fn ClearModelDataAsync<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearModelDataAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrainingStepsCompleted<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrainingStepsCompleted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrainingStepsRemaining<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrainingStepsRemaining() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrainingDataFormat<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionTrainingDataFormat) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrainingDataFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyTrainingData<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: ::windows::core::RawPtr, result__: *mut DetectionConfigurationTrainingStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyTrainingData(trainingdataformat, &*(&trainingdata as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyTrainingDataAsync<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyTrainingDataAsync(trainingdataformat, &*(&trainingdata as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearTrainingData<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearTrainingData().into()
        }
        unsafe extern "system" fn ClearTrainingDataAsync<Impl: IActivationSignalDetectionConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearTrainingDataAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IActivationSignalDetectionConfiguration>,
            ::windows::core::GetTrustLevel,
            SignalId::<Impl, IMPL_OFFSET>,
            ModelId::<Impl, IMPL_OFFSET>,
            DisplayName::<Impl, IMPL_OFFSET>,
            IsActive::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            SetEnabledAsync::<Impl, IMPL_OFFSET>,
            AvailabilityInfo::<Impl, IMPL_OFFSET>,
            AvailabilityChanged::<Impl, IMPL_OFFSET>,
            RemoveAvailabilityChanged::<Impl, IMPL_OFFSET>,
            SetModelData::<Impl, IMPL_OFFSET>,
            SetModelDataAsync::<Impl, IMPL_OFFSET>,
            GetModelDataType::<Impl, IMPL_OFFSET>,
            GetModelDataTypeAsync::<Impl, IMPL_OFFSET>,
            GetModelData::<Impl, IMPL_OFFSET>,
            GetModelDataAsync::<Impl, IMPL_OFFSET>,
            ClearModelData::<Impl, IMPL_OFFSET>,
            ClearModelDataAsync::<Impl, IMPL_OFFSET>,
            TrainingStepsCompleted::<Impl, IMPL_OFFSET>,
            TrainingStepsRemaining::<Impl, IMPL_OFFSET>,
            TrainingDataFormat::<Impl, IMPL_OFFSET>,
            ApplyTrainingData::<Impl, IMPL_OFFSET>,
            ApplyTrainingDataAsync::<Impl, IMPL_OFFSET>,
            ClearTrainingData::<Impl, IMPL_OFFSET>,
            ClearTrainingDataAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationSignalDetectionConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IActivationSignalDetectionConfiguration2Impl: Sized {
    fn SetModelDataWithResult(&self, datatype: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<ActivationSignalDetectionConfigurationSetModelDataResult>;
    fn SetModelDataWithResultAsync(&self, datatype: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>>;
    fn SetEnabledWithResultAsync(&self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>>;
    fn SetEnabledWithResult(&self, value: bool) -> ::windows::core::Result<ActivationSignalDetectionConfigurationStateChangeResult>;
    fn TrainingStepCompletionMaxAllowedTime(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivationSignalDetectionConfiguration2 {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IActivationSignalDetectionConfiguration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationSignalDetectionConfiguration2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationSignalDetectionConfiguration2Vtbl {
        unsafe extern "system" fn SetModelDataWithResult<Impl: IActivationSignalDetectionConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ActivationSignalDetectionConfigurationSetModelDataResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetModelDataWithResult(&*(&datatype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModelDataWithResultAsync<Impl: IActivationSignalDetectionConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetModelDataWithResultAsync(&*(&datatype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabledWithResultAsync<Impl: IActivationSignalDetectionConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabledWithResultAsync(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabledWithResult<Impl: IActivationSignalDetectionConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ActivationSignalDetectionConfigurationStateChangeResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabledWithResult(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrainingStepCompletionMaxAllowedTime<Impl: IActivationSignalDetectionConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrainingStepCompletionMaxAllowedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IActivationSignalDetectionConfiguration2>,
            ::windows::core::GetTrustLevel,
            SetModelDataWithResult::<Impl, IMPL_OFFSET>,
            SetModelDataWithResultAsync::<Impl, IMPL_OFFSET>,
            SetEnabledWithResultAsync::<Impl, IMPL_OFFSET>,
            SetEnabledWithResult::<Impl, IMPL_OFFSET>,
            TrainingStepCompletionMaxAllowedTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationSignalDetectionConfiguration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivationSignalDetectionConfigurationCreationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ActivationSignalDetectionConfigurationCreationStatus>;
    fn Configuration(&self) -> ::windows::core::Result<ActivationSignalDetectionConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivationSignalDetectionConfigurationCreationResult {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfigurationCreationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IActivationSignalDetectionConfigurationCreationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationSignalDetectionConfigurationCreationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationSignalDetectionConfigurationCreationResultVtbl {
        unsafe extern "system" fn Status<Impl: IActivationSignalDetectionConfigurationCreationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionConfigurationCreationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configuration<Impl: IActivationSignalDetectionConfigurationCreationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActivationSignalDetectionConfigurationCreationResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, Configuration::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationSignalDetectionConfigurationCreationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IActivationSignalDetectorImpl: Sized {
    fn ProviderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<ActivationSignalDetectorKind>;
    fn CanCreateConfigurations(&self) -> ::windows::core::Result<bool>;
    fn SupportedModelDataTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SupportedTrainingDataFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>>;
    fn SupportedPowerStates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>>;
    fn GetSupportedModelIdsForSignalId(&self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GetSupportedModelIdsForSignalIdAsync(&self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn CreateConfiguration(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CreateConfigurationAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetConfigurations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>;
    fn GetConfigurationsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>>;
    fn GetConfiguration(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetectionConfiguration>;
    fn GetConfigurationAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>>;
    fn RemoveConfiguration(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveConfigurationAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivationSignalDetector {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IActivationSignalDetectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationSignalDetectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationSignalDetectorVtbl {
        unsafe extern "system" fn ProviderId<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectorKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanCreateConfigurations<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanCreateConfigurations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModelDataTypes<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedModelDataTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTrainingDataFormats<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedTrainingDataFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedPowerStates<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedPowerStates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedModelIdsForSignalId<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedModelIdsForSignalId(&*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedModelIdsForSignalIdAsync<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedModelIdsForSignalIdAsync(&*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConfiguration<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CreateConfiguration(
                    &*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&modelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn CreateConfigurationAsync<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateConfigurationAsync(
                &*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&modelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfigurations<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfigurations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfigurationsAsync<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfigurationsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfiguration<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfiguration(&*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&modelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfigurationAsync<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConfigurationAsync(&*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&modelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConfiguration<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConfiguration(&*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&modelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveConfigurationAsync<Impl: IActivationSignalDetectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveConfigurationAsync(&*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&modelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IActivationSignalDetector>,
            ::windows::core::GetTrustLevel,
            ProviderId::<Impl, IMPL_OFFSET>,
            Kind::<Impl, IMPL_OFFSET>,
            CanCreateConfigurations::<Impl, IMPL_OFFSET>,
            SupportedModelDataTypes::<Impl, IMPL_OFFSET>,
            SupportedTrainingDataFormats::<Impl, IMPL_OFFSET>,
            SupportedPowerStates::<Impl, IMPL_OFFSET>,
            GetSupportedModelIdsForSignalId::<Impl, IMPL_OFFSET>,
            GetSupportedModelIdsForSignalIdAsync::<Impl, IMPL_OFFSET>,
            CreateConfiguration::<Impl, IMPL_OFFSET>,
            CreateConfigurationAsync::<Impl, IMPL_OFFSET>,
            GetConfigurations::<Impl, IMPL_OFFSET>,
            GetConfigurationsAsync::<Impl, IMPL_OFFSET>,
            GetConfiguration::<Impl, IMPL_OFFSET>,
            GetConfigurationAsync::<Impl, IMPL_OFFSET>,
            RemoveConfiguration::<Impl, IMPL_OFFSET>,
            RemoveConfigurationAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationSignalDetector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IActivationSignalDetector2Impl: Sized {
    fn GetAvailableModelIdsForSignalIdAsync(&self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn GetAvailableModelIdsForSignalId(&self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn CreateConfigurationWithResultAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>>;
    fn CreateConfigurationWithResult(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetectionConfigurationCreationResult>;
    fn RemoveConfigurationWithResultAsync(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>>;
    fn RemoveConfigurationWithResult(&self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetectionConfigurationRemovalResult>;
    fn DetectorId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivationSignalDetector2 {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IActivationSignalDetector2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationSignalDetector2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationSignalDetector2Vtbl {
        unsafe extern "system" fn GetAvailableModelIdsForSignalIdAsync<Impl: IActivationSignalDetector2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableModelIdsForSignalIdAsync(&*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableModelIdsForSignalId<Impl: IActivationSignalDetector2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableModelIdsForSignalId(&*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConfigurationWithResultAsync<Impl: IActivationSignalDetector2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateConfigurationWithResultAsync(
                &*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&modelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConfigurationWithResult<Impl: IActivationSignalDetector2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateConfigurationWithResult(
                &*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&modelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConfigurationWithResultAsync<Impl: IActivationSignalDetector2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveConfigurationWithResultAsync(&*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&modelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConfigurationWithResult<Impl: IActivationSignalDetector2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ActivationSignalDetectionConfigurationRemovalResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveConfigurationWithResult(&*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&modelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetectorId<Impl: IActivationSignalDetector2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetectorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IActivationSignalDetector2>,
            ::windows::core::GetTrustLevel,
            GetAvailableModelIdsForSignalIdAsync::<Impl, IMPL_OFFSET>,
            GetAvailableModelIdsForSignalId::<Impl, IMPL_OFFSET>,
            CreateConfigurationWithResultAsync::<Impl, IMPL_OFFSET>,
            CreateConfigurationWithResult::<Impl, IMPL_OFFSET>,
            RemoveConfigurationWithResultAsync::<Impl, IMPL_OFFSET>,
            RemoveConfigurationWithResult::<Impl, IMPL_OFFSET>,
            DetectorId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationSignalDetector2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IConversationalAgentDetectorManagerImpl: Sized {
    fn GetAllActivationSignalDetectors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>;
    fn GetAllActivationSignalDetectorsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>>;
    fn GetActivationSignalDetectors(&self, kind: ActivationSignalDetectorKind) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>;
    fn GetActivationSignalDetectorsAsync(&self, kind: ActivationSignalDetectorKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConversationalAgentDetectorManager {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IConversationalAgentDetectorManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentDetectorManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentDetectorManagerVtbl {
        unsafe extern "system" fn GetAllActivationSignalDetectors<Impl: IConversationalAgentDetectorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllActivationSignalDetectors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllActivationSignalDetectorsAsync<Impl: IConversationalAgentDetectorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllActivationSignalDetectorsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivationSignalDetectors<Impl: IConversationalAgentDetectorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: ActivationSignalDetectorKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivationSignalDetectors(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivationSignalDetectorsAsync<Impl: IConversationalAgentDetectorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: ActivationSignalDetectorKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivationSignalDetectorsAsync(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IConversationalAgentDetectorManager>,
            ::windows::core::GetTrustLevel,
            GetAllActivationSignalDetectors::<Impl, IMPL_OFFSET>,
            GetAllActivationSignalDetectorsAsync::<Impl, IMPL_OFFSET>,
            GetActivationSignalDetectors::<Impl, IMPL_OFFSET>,
            GetActivationSignalDetectorsAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentDetectorManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConversationalAgentDetectorManager2Impl: Sized {
    fn GetActivationSignalDetectorFromId(&self, detectorid: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetector>;
    fn GetActivationSignalDetectorFromIdAsync(&self, detectorid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetector>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConversationalAgentDetectorManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConversationalAgentDetectorManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentDetectorManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentDetectorManager2Vtbl {
        unsafe extern "system" fn GetActivationSignalDetectorFromId<Impl: IConversationalAgentDetectorManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, detectorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivationSignalDetectorFromId(&*(&detectorid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivationSignalDetectorFromIdAsync<Impl: IConversationalAgentDetectorManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, detectorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivationSignalDetectorFromIdAsync(&*(&detectorid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConversationalAgentDetectorManager2>, ::windows::core::GetTrustLevel, GetActivationSignalDetectorFromId::<Impl, IMPL_OFFSET>, GetActivationSignalDetectorFromIdAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentDetectorManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentDetectorManagerStaticsImpl: Sized {
    fn Default(&self) -> ::windows::core::Result<ConversationalAgentDetectorManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConversationalAgentDetectorManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IConversationalAgentDetectorManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentDetectorManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentDetectorManagerStaticsVtbl {
        unsafe extern "system" fn Default<Impl: IConversationalAgentDetectorManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Default() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConversationalAgentDetectorManagerStatics>, ::windows::core::GetTrustLevel, Default::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentDetectorManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Audio", feature = "implement_exclusive"))]
pub trait IConversationalAgentSessionImpl: Sized {
    fn SessionInterrupted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSessionInterruptedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionInterrupted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SignalDetected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSignalDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSignalDetected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SystemStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSystemStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AgentState(&self) -> ::windows::core::Result<ConversationalAgentState>;
    fn Signal(&self) -> ::windows::core::Result<ConversationalAgentSignal>;
    fn IsIndicatorLightAvailable(&self) -> ::windows::core::Result<bool>;
    fn IsScreenAvailable(&self) -> ::windows::core::Result<bool>;
    fn IsUserAuthenticated(&self) -> ::windows::core::Result<bool>;
    fn IsVoiceActivationAvailable(&self) -> ::windows::core::Result<bool>;
    fn IsInterruptible(&self) -> ::windows::core::Result<bool>;
    fn IsInterrupted(&self) -> ::windows::core::Result<bool>;
    fn RequestInterruptibleAsync(&self, interruptible: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>;
    fn RequestInterruptible(&self, interruptible: bool) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse>;
    fn RequestAgentStateChangeAsync(&self, state: ConversationalAgentState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>;
    fn RequestAgentStateChange(&self, state: ConversationalAgentState) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse>;
    fn RequestForegroundActivationAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>;
    fn RequestForegroundActivation(&self) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse>;
    fn GetAudioClientAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
    fn GetAudioClient(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateAudioDeviceInputNodeAsync(&self, graph: &::core::option::Option<super::super::Media::Audio::AudioGraph>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Media::Audio::AudioDeviceInputNode>>;
    fn CreateAudioDeviceInputNode(&self, graph: &::core::option::Option<super::super::Media::Audio::AudioGraph>) -> ::windows::core::Result<super::super::Media::Audio::AudioDeviceInputNode>;
    fn GetAudioCaptureDeviceIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetAudioCaptureDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAudioRenderDeviceIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetAudioRenderDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetSignalModelIdAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetSignalModelId(&self) -> ::windows::core::Result<u32>;
    fn SetSignalModelIdAsync(&self, signalmodelid: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SetSignalModelId(&self, signalmodelid: u32) -> ::windows::core::Result<bool>;
    fn GetSupportedSignalModelIdsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>>;
    fn GetSupportedSignalModelIds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Audio", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConversationalAgentSession {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Audio", feature = "implement_exclusive"))]
impl IConversationalAgentSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSessionVtbl {
        unsafe extern "system" fn SessionInterrupted<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionInterrupted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSessionInterruptedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSessionInterruptedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSessionInterrupted<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSessionInterrupted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignalDetected<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalDetected(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSignalDetectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSignalDetectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSignalDetected<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSignalDetected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemStateChanged<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemStateChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSystemStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSystemStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSystemStateChanged<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSystemStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AgentState<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AgentState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Signal<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Signal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsIndicatorLightAvailable<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsIndicatorLightAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsScreenAvailable<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsScreenAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserAuthenticated<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUserAuthenticated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsVoiceActivationAvailable<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsVoiceActivationAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInterruptible<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInterruptible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInterrupted<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInterrupted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestInterruptibleAsync<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interruptible: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestInterruptibleAsync(interruptible) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestInterruptible<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interruptible: bool, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestInterruptible(interruptible) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAgentStateChangeAsync<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: ConversationalAgentState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAgentStateChangeAsync(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAgentStateChange<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: ConversationalAgentState, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAgentStateChange(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestForegroundActivationAsync<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestForegroundActivationAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestForegroundActivation<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestForegroundActivation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioClientAsync<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioClientAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioClient<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioClient() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioDeviceInputNodeAsync<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, graph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAudioDeviceInputNodeAsync(&*(&graph as *const <super::super::Media::Audio::AudioGraph as ::windows::core::Abi>::Abi as *const <super::super::Media::Audio::AudioGraph as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioDeviceInputNode<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, graph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAudioDeviceInputNode(&*(&graph as *const <super::super::Media::Audio::AudioGraph as ::windows::core::Abi>::Abi as *const <super::super::Media::Audio::AudioGraph as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioCaptureDeviceIdAsync<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioCaptureDeviceIdAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioCaptureDeviceId<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioCaptureDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioRenderDeviceIdAsync<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioRenderDeviceIdAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioRenderDeviceId<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioRenderDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalModelIdAsync<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignalModelIdAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalModelId<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignalModelId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignalModelIdAsync<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalmodelid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSignalModelIdAsync(signalmodelid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignalModelId<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalmodelid: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSignalModelId(signalmodelid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedSignalModelIdsAsync<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedSignalModelIdsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedSignalModelIds<Impl: IConversationalAgentSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedSignalModelIds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IConversationalAgentSession>,
            ::windows::core::GetTrustLevel,
            SessionInterrupted::<Impl, IMPL_OFFSET>,
            RemoveSessionInterrupted::<Impl, IMPL_OFFSET>,
            SignalDetected::<Impl, IMPL_OFFSET>,
            RemoveSignalDetected::<Impl, IMPL_OFFSET>,
            SystemStateChanged::<Impl, IMPL_OFFSET>,
            RemoveSystemStateChanged::<Impl, IMPL_OFFSET>,
            AgentState::<Impl, IMPL_OFFSET>,
            Signal::<Impl, IMPL_OFFSET>,
            IsIndicatorLightAvailable::<Impl, IMPL_OFFSET>,
            IsScreenAvailable::<Impl, IMPL_OFFSET>,
            IsUserAuthenticated::<Impl, IMPL_OFFSET>,
            IsVoiceActivationAvailable::<Impl, IMPL_OFFSET>,
            IsInterruptible::<Impl, IMPL_OFFSET>,
            IsInterrupted::<Impl, IMPL_OFFSET>,
            RequestInterruptibleAsync::<Impl, IMPL_OFFSET>,
            RequestInterruptible::<Impl, IMPL_OFFSET>,
            RequestAgentStateChangeAsync::<Impl, IMPL_OFFSET>,
            RequestAgentStateChange::<Impl, IMPL_OFFSET>,
            RequestForegroundActivationAsync::<Impl, IMPL_OFFSET>,
            RequestForegroundActivation::<Impl, IMPL_OFFSET>,
            GetAudioClientAsync::<Impl, IMPL_OFFSET>,
            GetAudioClient::<Impl, IMPL_OFFSET>,
            CreateAudioDeviceInputNodeAsync::<Impl, IMPL_OFFSET>,
            CreateAudioDeviceInputNode::<Impl, IMPL_OFFSET>,
            GetAudioCaptureDeviceIdAsync::<Impl, IMPL_OFFSET>,
            GetAudioCaptureDeviceId::<Impl, IMPL_OFFSET>,
            GetAudioRenderDeviceIdAsync::<Impl, IMPL_OFFSET>,
            GetAudioRenderDeviceId::<Impl, IMPL_OFFSET>,
            GetSignalModelIdAsync::<Impl, IMPL_OFFSET>,
            GetSignalModelId::<Impl, IMPL_OFFSET>,
            SetSignalModelIdAsync::<Impl, IMPL_OFFSET>,
            SetSignalModelId::<Impl, IMPL_OFFSET>,
            GetSupportedSignalModelIdsAsync::<Impl, IMPL_OFFSET>,
            GetSupportedSignalModelIds::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IConversationalAgentSession2Impl: Sized {
    fn RequestActivationAsync(&self, activationkind: ConversationalAgentActivationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentActivationResult>>;
    fn RequestActivation(&self, activationkind: ConversationalAgentActivationKind) -> ::windows::core::Result<ConversationalAgentActivationResult>;
    fn SetSupportLockScreenActivationAsync(&self, lockscreenactivationsupported: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetSupportLockScreenActivation(&self, lockscreenactivationsupported: bool) -> ::windows::core::Result<()>;
    fn GetMissingPrerequisites(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>;
    fn GetMissingPrerequisitesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConversationalAgentSession2 {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IConversationalAgentSession2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSession2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSession2Vtbl {
        unsafe extern "system" fn RequestActivationAsync<Impl: IConversationalAgentSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activationkind: ConversationalAgentActivationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestActivationAsync(activationkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestActivation<Impl: IConversationalAgentSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activationkind: ConversationalAgentActivationKind, result__: *mut ConversationalAgentActivationResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestActivation(activationkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportLockScreenActivationAsync<Impl: IConversationalAgentSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lockscreenactivationsupported: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSupportLockScreenActivationAsync(lockscreenactivationsupported) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSupportLockScreenActivation<Impl: IConversationalAgentSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lockscreenactivationsupported: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportLockScreenActivation(lockscreenactivationsupported).into()
        }
        unsafe extern "system" fn GetMissingPrerequisites<Impl: IConversationalAgentSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMissingPrerequisites() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMissingPrerequisitesAsync<Impl: IConversationalAgentSession2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMissingPrerequisitesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IConversationalAgentSession2>,
            ::windows::core::GetTrustLevel,
            RequestActivationAsync::<Impl, IMPL_OFFSET>,
            RequestActivation::<Impl, IMPL_OFFSET>,
            SetSupportLockScreenActivationAsync::<Impl, IMPL_OFFSET>,
            SetSupportLockScreenActivation::<Impl, IMPL_OFFSET>,
            GetMissingPrerequisites::<Impl, IMPL_OFFSET>,
            GetMissingPrerequisitesAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSessionInterruptedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConversationalAgentSessionInterruptedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionInterruptedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IConversationalAgentSessionInterruptedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSessionInterruptedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSessionInterruptedEventArgsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConversationalAgentSessionInterruptedEventArgs>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSessionInterruptedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConversationalAgentSessionStaticsImpl: Sized {
    fn GetCurrentSessionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSession>>;
    fn GetCurrentSessionSync(&self) -> ::windows::core::Result<ConversationalAgentSession>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConversationalAgentSessionStatics {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConversationalAgentSessionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSessionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSessionStaticsVtbl {
        unsafe extern "system" fn GetCurrentSessionAsync<Impl: IConversationalAgentSessionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSessionAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSessionSync<Impl: IConversationalAgentSessionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSessionSync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConversationalAgentSessionStatics>, ::windows::core::GetTrustLevel, GetCurrentSessionAsync::<Impl, IMPL_OFFSET>, GetCurrentSessionSync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSessionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConversationalAgentSignalImpl: Sized {
    fn IsSignalVerificationRequired(&self) -> ::windows::core::Result<bool>;
    fn SetIsSignalVerificationRequired(&self, value: bool) -> ::windows::core::Result<()>;
    fn SignalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSignalId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SignalName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSignalName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SignalContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSignalContext(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SignalStart(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetSignalStart(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SignalEnd(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetSignalEnd(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConversationalAgentSignal {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConversationalAgentSignalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSignalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSignalVtbl {
        unsafe extern "system" fn IsSignalVerificationRequired<Impl: IConversationalAgentSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSignalVerificationRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSignalVerificationRequired<Impl: IConversationalAgentSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSignalVerificationRequired(value).into()
        }
        unsafe extern "system" fn SignalId<Impl: IConversationalAgentSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignalId<Impl: IConversationalAgentSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignalId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignalName<Impl: IConversationalAgentSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignalName<Impl: IConversationalAgentSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignalName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignalContext<Impl: IConversationalAgentSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignalContext<Impl: IConversationalAgentSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignalContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignalStart<Impl: IConversationalAgentSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalStart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignalStart<Impl: IConversationalAgentSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignalStart(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignalEnd<Impl: IConversationalAgentSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignalEnd<Impl: IConversationalAgentSignalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignalEnd(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IConversationalAgentSignal>,
            ::windows::core::GetTrustLevel,
            IsSignalVerificationRequired::<Impl, IMPL_OFFSET>,
            SetIsSignalVerificationRequired::<Impl, IMPL_OFFSET>,
            SignalId::<Impl, IMPL_OFFSET>,
            SetSignalId::<Impl, IMPL_OFFSET>,
            SignalName::<Impl, IMPL_OFFSET>,
            SetSignalName::<Impl, IMPL_OFFSET>,
            SignalContext::<Impl, IMPL_OFFSET>,
            SetSignalContext::<Impl, IMPL_OFFSET>,
            SignalStart::<Impl, IMPL_OFFSET>,
            SetSignalStart::<Impl, IMPL_OFFSET>,
            SignalEnd::<Impl, IMPL_OFFSET>,
            SetSignalEnd::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSignal as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSignal2Impl: Sized {
    fn DetectorId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DetectorKind(&self) -> ::windows::core::Result<ActivationSignalDetectorKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConversationalAgentSignal2 {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal2";
}
#[cfg(feature = "implement_exclusive")]
impl IConversationalAgentSignal2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSignal2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSignal2Vtbl {
        unsafe extern "system" fn DetectorId<Impl: IConversationalAgentSignal2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetectorId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetectorKind<Impl: IConversationalAgentSignal2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectorKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetectorKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConversationalAgentSignal2>, ::windows::core::GetTrustLevel, DetectorId::<Impl, IMPL_OFFSET>, DetectorKind::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSignal2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSignalDetectedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConversationalAgentSignalDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignalDetectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IConversationalAgentSignalDetectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSignalDetectedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSignalDetectedEventArgsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConversationalAgentSignalDetectedEventArgs>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSignalDetectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSystemStateChangedEventArgsImpl: Sized {
    fn SystemStateChangeType(&self) -> ::windows::core::Result<ConversationalAgentSystemStateChangeType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConversationalAgentSystemStateChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSystemStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IConversationalAgentSystemStateChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSystemStateChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSystemStateChangedEventArgsVtbl {
        unsafe extern "system" fn SystemStateChangeType<Impl: IConversationalAgentSystemStateChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentSystemStateChangeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemStateChangeType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IConversationalAgentSystemStateChangedEventArgs>, ::windows::core::GetTrustLevel, SystemStateChangeType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSystemStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDetectionConfigurationAvailabilityChangedEventArgsImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<DetectionConfigurationAvailabilityChangeKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDetectionConfigurationAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDetectionConfigurationAvailabilityChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDetectionConfigurationAvailabilityChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDetectionConfigurationAvailabilityChangedEventArgsVtbl {
        unsafe extern "system" fn Kind<Impl: IDetectionConfigurationAvailabilityChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DetectionConfigurationAvailabilityChangeKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDetectionConfigurationAvailabilityChangedEventArgs>, ::windows::core::GetTrustLevel, Kind::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDetectionConfigurationAvailabilityChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDetectionConfigurationAvailabilityInfoImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn HasSystemResourceAccess(&self) -> ::windows::core::Result<bool>;
    fn HasPermission(&self) -> ::windows::core::Result<bool>;
    fn HasLockScreenPermission(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDetectionConfigurationAvailabilityInfo {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IDetectionConfigurationAvailabilityInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDetectionConfigurationAvailabilityInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDetectionConfigurationAvailabilityInfoVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IDetectionConfigurationAvailabilityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasSystemResourceAccess<Impl: IDetectionConfigurationAvailabilityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasSystemResourceAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasPermission<Impl: IDetectionConfigurationAvailabilityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasPermission() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasLockScreenPermission<Impl: IDetectionConfigurationAvailabilityInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasLockScreenPermission() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDetectionConfigurationAvailabilityInfo>, ::windows::core::GetTrustLevel, IsEnabled::<Impl, IMPL_OFFSET>, HasSystemResourceAccess::<Impl, IMPL_OFFSET>, HasPermission::<Impl, IMPL_OFFSET>, HasLockScreenPermission::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDetectionConfigurationAvailabilityInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDetectionConfigurationAvailabilityInfo2Impl: Sized {
    fn UnavailableSystemResources(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SignalDetectorResourceKind>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDetectionConfigurationAvailabilityInfo2 {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDetectionConfigurationAvailabilityInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDetectionConfigurationAvailabilityInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDetectionConfigurationAvailabilityInfo2Vtbl {
        unsafe extern "system" fn UnavailableSystemResources<Impl: IDetectionConfigurationAvailabilityInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnavailableSystemResources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDetectionConfigurationAvailabilityInfo2>, ::windows::core::GetTrustLevel, UnavailableSystemResources::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDetectionConfigurationAvailabilityInfo2 as ::windows::core::Interface>::IID
    }
}
