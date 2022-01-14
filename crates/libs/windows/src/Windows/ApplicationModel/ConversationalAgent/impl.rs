#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IActivationSignalDetectionConfiguration_Impl: Sized {
    fn SignalId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsActive(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SetEnabledAsync(&mut self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn AvailabilityInfo(&mut self) -> ::windows::core::Result<DetectionConfigurationAvailabilityInfo>;
    fn AvailabilityChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ActivationSignalDetectionConfiguration, DetectionConfigurationAvailabilityChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailabilityChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetModelData(&mut self, datatype: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn SetModelDataAsync(&mut self, datatype: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetModelDataType(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetModelDataTypeAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetModelData(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn GetModelDataAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IInputStream>>;
    fn ClearModelData(&mut self) -> ::windows::core::Result<()>;
    fn ClearModelDataAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TrainingStepsCompleted(&mut self) -> ::windows::core::Result<u32>;
    fn TrainingStepsRemaining(&mut self) -> ::windows::core::Result<u32>;
    fn TrainingDataFormat(&mut self) -> ::windows::core::Result<ActivationSignalDetectionTrainingDataFormat>;
    fn ApplyTrainingData(&mut self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<DetectionConfigurationTrainingStatus>;
    fn ApplyTrainingDataAsync(&mut self, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DetectionConfigurationTrainingStatus>>;
    fn ClearTrainingData(&mut self) -> ::windows::core::Result<()>;
    fn ClearTrainingDataAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivationSignalDetectionConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IActivationSignalDetectionConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationSignalDetectionConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationSignalDetectionConfiguration_Vtbl {
        unsafe extern "system" fn SignalId<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ModelId<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsActive<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnabled<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(value).into()
        }
        unsafe extern "system" fn SetEnabledAsync<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AvailabilityInfo<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AvailabilityChanged<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAvailabilityChanged<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAvailabilityChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetModelData<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModelData(&*(&datatype as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::Storage::Streams::IInputStream as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IInputStream as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetModelDataAsync<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetModelDataType<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetModelDataTypeAsync<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetModelData<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetModelDataAsync<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearModelData<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearModelData().into()
        }
        unsafe extern "system" fn ClearModelDataAsync<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrainingStepsCompleted<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrainingStepsRemaining<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrainingDataFormat<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionTrainingDataFormat) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ApplyTrainingData<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: ::windows::core::RawPtr, result__: *mut DetectionConfigurationTrainingStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ApplyTrainingDataAsync<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trainingdataformat: ActivationSignalDetectionTrainingDataFormat, trainingdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClearTrainingData<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearTrainingData().into()
        }
        unsafe extern "system" fn ClearTrainingDataAsync<Impl: IActivationSignalDetectionConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivationSignalDetectionConfiguration, BASE_OFFSET>(),
            SignalId: SignalId::<Impl, IMPL_OFFSET>,
            ModelId: ModelId::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            SetEnabledAsync: SetEnabledAsync::<Impl, IMPL_OFFSET>,
            AvailabilityInfo: AvailabilityInfo::<Impl, IMPL_OFFSET>,
            AvailabilityChanged: AvailabilityChanged::<Impl, IMPL_OFFSET>,
            RemoveAvailabilityChanged: RemoveAvailabilityChanged::<Impl, IMPL_OFFSET>,
            SetModelData: SetModelData::<Impl, IMPL_OFFSET>,
            SetModelDataAsync: SetModelDataAsync::<Impl, IMPL_OFFSET>,
            GetModelDataType: GetModelDataType::<Impl, IMPL_OFFSET>,
            GetModelDataTypeAsync: GetModelDataTypeAsync::<Impl, IMPL_OFFSET>,
            GetModelData: GetModelData::<Impl, IMPL_OFFSET>,
            GetModelDataAsync: GetModelDataAsync::<Impl, IMPL_OFFSET>,
            ClearModelData: ClearModelData::<Impl, IMPL_OFFSET>,
            ClearModelDataAsync: ClearModelDataAsync::<Impl, IMPL_OFFSET>,
            TrainingStepsCompleted: TrainingStepsCompleted::<Impl, IMPL_OFFSET>,
            TrainingStepsRemaining: TrainingStepsRemaining::<Impl, IMPL_OFFSET>,
            TrainingDataFormat: TrainingDataFormat::<Impl, IMPL_OFFSET>,
            ApplyTrainingData: ApplyTrainingData::<Impl, IMPL_OFFSET>,
            ApplyTrainingDataAsync: ApplyTrainingDataAsync::<Impl, IMPL_OFFSET>,
            ClearTrainingData: ClearTrainingData::<Impl, IMPL_OFFSET>,
            ClearTrainingDataAsync: ClearTrainingDataAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationSignalDetectionConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IActivationSignalDetectionConfiguration2_Impl: Sized {
    fn SetModelDataWithResult(&mut self, datatype: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<ActivationSignalDetectionConfigurationSetModelDataResult>;
    fn SetModelDataWithResultAsync(&mut self, datatype: &::windows::core::HSTRING, data: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationSetModelDataResult>>;
    fn SetEnabledWithResultAsync(&mut self, value: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationStateChangeResult>>;
    fn SetEnabledWithResult(&mut self, value: bool) -> ::windows::core::Result<ActivationSignalDetectionConfigurationStateChangeResult>;
    fn TrainingStepCompletionMaxAllowedTime(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivationSignalDetectionConfiguration2 {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfiguration2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IActivationSignalDetectionConfiguration2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationSignalDetectionConfiguration2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationSignalDetectionConfiguration2_Vtbl {
        unsafe extern "system" fn SetModelDataWithResult<Impl: IActivationSignalDetectionConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ActivationSignalDetectionConfigurationSetModelDataResult) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetModelDataWithResultAsync<Impl: IActivationSignalDetectionConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnabledWithResultAsync<Impl: IActivationSignalDetectionConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnabledWithResult<Impl: IActivationSignalDetectionConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut ActivationSignalDetectionConfigurationStateChangeResult) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrainingStepCompletionMaxAllowedTime<Impl: IActivationSignalDetectionConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivationSignalDetectionConfiguration2, BASE_OFFSET>(),
            SetModelDataWithResult: SetModelDataWithResult::<Impl, IMPL_OFFSET>,
            SetModelDataWithResultAsync: SetModelDataWithResultAsync::<Impl, IMPL_OFFSET>,
            SetEnabledWithResultAsync: SetEnabledWithResultAsync::<Impl, IMPL_OFFSET>,
            SetEnabledWithResult: SetEnabledWithResult::<Impl, IMPL_OFFSET>,
            TrainingStepCompletionMaxAllowedTime: TrainingStepCompletionMaxAllowedTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationSignalDetectionConfiguration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivationSignalDetectionConfigurationCreationResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<ActivationSignalDetectionConfigurationCreationStatus>;
    fn Configuration(&mut self) -> ::windows::core::Result<ActivationSignalDetectionConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IActivationSignalDetectionConfigurationCreationResult {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetectionConfigurationCreationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IActivationSignalDetectionConfigurationCreationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationSignalDetectionConfigurationCreationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationSignalDetectionConfigurationCreationResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IActivationSignalDetectionConfigurationCreationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectionConfigurationCreationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Configuration<Impl: IActivationSignalDetectionConfigurationCreationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivationSignalDetectionConfigurationCreationResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationSignalDetectionConfigurationCreationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IActivationSignalDetector_Impl: Sized {
    fn ProviderId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&mut self) -> ::windows::core::Result<ActivationSignalDetectorKind>;
    fn CanCreateConfigurations(&mut self) -> ::windows::core::Result<bool>;
    fn SupportedModelDataTypes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SupportedTrainingDataFormats(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionTrainingDataFormat>>;
    fn SupportedPowerStates(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectorPowerState>>;
    fn GetSupportedModelIdsForSignalId(&mut self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GetSupportedModelIdsForSignalIdAsync(&mut self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn CreateConfiguration(&mut self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CreateConfigurationAsync(&mut self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetConfigurations(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>;
    fn GetConfigurationsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetectionConfiguration>>>;
    fn GetConfiguration(&mut self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetectionConfiguration>;
    fn GetConfigurationAsync(&mut self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfiguration>>;
    fn RemoveConfiguration(&mut self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveConfigurationAsync(&mut self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivationSignalDetector {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IActivationSignalDetector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationSignalDetector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationSignalDetector_Vtbl {
        unsafe extern "system" fn ProviderId<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Kind<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectorKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanCreateConfigurations<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedModelDataTypes<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedTrainingDataFormats<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedPowerStates<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSupportedModelIdsForSignalId<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSupportedModelIdsForSignalIdAsync<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateConfiguration<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .CreateConfiguration(
                    &*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&modelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn CreateConfigurationAsync<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetConfigurations<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetConfigurationsAsync<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetConfiguration<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetConfigurationAsync<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveConfiguration<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConfiguration(&*(&signalid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&modelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveConfigurationAsync<Impl: IActivationSignalDetector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivationSignalDetector, BASE_OFFSET>(),
            ProviderId: ProviderId::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            CanCreateConfigurations: CanCreateConfigurations::<Impl, IMPL_OFFSET>,
            SupportedModelDataTypes: SupportedModelDataTypes::<Impl, IMPL_OFFSET>,
            SupportedTrainingDataFormats: SupportedTrainingDataFormats::<Impl, IMPL_OFFSET>,
            SupportedPowerStates: SupportedPowerStates::<Impl, IMPL_OFFSET>,
            GetSupportedModelIdsForSignalId: GetSupportedModelIdsForSignalId::<Impl, IMPL_OFFSET>,
            GetSupportedModelIdsForSignalIdAsync: GetSupportedModelIdsForSignalIdAsync::<Impl, IMPL_OFFSET>,
            CreateConfiguration: CreateConfiguration::<Impl, IMPL_OFFSET>,
            CreateConfigurationAsync: CreateConfigurationAsync::<Impl, IMPL_OFFSET>,
            GetConfigurations: GetConfigurations::<Impl, IMPL_OFFSET>,
            GetConfigurationsAsync: GetConfigurationsAsync::<Impl, IMPL_OFFSET>,
            GetConfiguration: GetConfiguration::<Impl, IMPL_OFFSET>,
            GetConfigurationAsync: GetConfigurationAsync::<Impl, IMPL_OFFSET>,
            RemoveConfiguration: RemoveConfiguration::<Impl, IMPL_OFFSET>,
            RemoveConfigurationAsync: RemoveConfigurationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationSignalDetector as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IActivationSignalDetector2_Impl: Sized {
    fn GetAvailableModelIdsForSignalIdAsync(&mut self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>>;
    fn GetAvailableModelIdsForSignalId(&mut self, signalid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn CreateConfigurationWithResultAsync(&mut self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationCreationResult>>;
    fn CreateConfigurationWithResult(&mut self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetectionConfigurationCreationResult>;
    fn RemoveConfigurationWithResultAsync(&mut self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetectionConfigurationRemovalResult>>;
    fn RemoveConfigurationWithResult(&mut self, signalid: &::windows::core::HSTRING, modelid: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetectionConfigurationRemovalResult>;
    fn DetectorId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IActivationSignalDetector2 {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IActivationSignalDetector2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IActivationSignalDetector2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationSignalDetector2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationSignalDetector2_Vtbl {
        unsafe extern "system" fn GetAvailableModelIdsForSignalIdAsync<Impl: IActivationSignalDetector2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAvailableModelIdsForSignalId<Impl: IActivationSignalDetector2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateConfigurationWithResultAsync<Impl: IActivationSignalDetector2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateConfigurationWithResult<Impl: IActivationSignalDetector2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveConfigurationWithResultAsync<Impl: IActivationSignalDetector2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveConfigurationWithResult<Impl: IActivationSignalDetector2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, modelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ActivationSignalDetectionConfigurationRemovalResult) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DetectorId<Impl: IActivationSignalDetector2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivationSignalDetector2, BASE_OFFSET>(),
            GetAvailableModelIdsForSignalIdAsync: GetAvailableModelIdsForSignalIdAsync::<Impl, IMPL_OFFSET>,
            GetAvailableModelIdsForSignalId: GetAvailableModelIdsForSignalId::<Impl, IMPL_OFFSET>,
            CreateConfigurationWithResultAsync: CreateConfigurationWithResultAsync::<Impl, IMPL_OFFSET>,
            CreateConfigurationWithResult: CreateConfigurationWithResult::<Impl, IMPL_OFFSET>,
            RemoveConfigurationWithResultAsync: RemoveConfigurationWithResultAsync::<Impl, IMPL_OFFSET>,
            RemoveConfigurationWithResult: RemoveConfigurationWithResult::<Impl, IMPL_OFFSET>,
            DetectorId: DetectorId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationSignalDetector2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IConversationalAgentDetectorManager_Impl: Sized {
    fn GetAllActivationSignalDetectors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>;
    fn GetAllActivationSignalDetectorsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>>;
    fn GetActivationSignalDetectors(&mut self, kind: ActivationSignalDetectorKind) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>;
    fn GetActivationSignalDetectorsAsync(&mut self, kind: ActivationSignalDetectorKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ActivationSignalDetector>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConversationalAgentDetectorManager {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IConversationalAgentDetectorManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentDetectorManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentDetectorManager_Vtbl {
        unsafe extern "system" fn GetAllActivationSignalDetectors<Impl: IConversationalAgentDetectorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAllActivationSignalDetectorsAsync<Impl: IConversationalAgentDetectorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetActivationSignalDetectors<Impl: IConversationalAgentDetectorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: ActivationSignalDetectorKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetActivationSignalDetectorsAsync<Impl: IConversationalAgentDetectorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: ActivationSignalDetectorKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConversationalAgentDetectorManager, BASE_OFFSET>(),
            GetAllActivationSignalDetectors: GetAllActivationSignalDetectors::<Impl, IMPL_OFFSET>,
            GetAllActivationSignalDetectorsAsync: GetAllActivationSignalDetectorsAsync::<Impl, IMPL_OFFSET>,
            GetActivationSignalDetectors: GetActivationSignalDetectors::<Impl, IMPL_OFFSET>,
            GetActivationSignalDetectorsAsync: GetActivationSignalDetectorsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentDetectorManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConversationalAgentDetectorManager2_Impl: Sized {
    fn GetActivationSignalDetectorFromId(&mut self, detectorid: &::windows::core::HSTRING) -> ::windows::core::Result<ActivationSignalDetector>;
    fn GetActivationSignalDetectorFromIdAsync(&mut self, detectorid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ActivationSignalDetector>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConversationalAgentDetectorManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManager2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConversationalAgentDetectorManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentDetectorManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentDetectorManager2_Vtbl {
        unsafe extern "system" fn GetActivationSignalDetectorFromId<Impl: IConversationalAgentDetectorManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, detectorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetActivationSignalDetectorFromIdAsync<Impl: IConversationalAgentDetectorManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, detectorid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConversationalAgentDetectorManager2, BASE_OFFSET>(),
            GetActivationSignalDetectorFromId: GetActivationSignalDetectorFromId::<Impl, IMPL_OFFSET>,
            GetActivationSignalDetectorFromIdAsync: GetActivationSignalDetectorFromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentDetectorManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentDetectorManagerStatics_Impl: Sized {
    fn Default(&mut self) -> ::windows::core::Result<ConversationalAgentDetectorManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConversationalAgentDetectorManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentDetectorManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IConversationalAgentDetectorManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentDetectorManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentDetectorManagerStatics_Vtbl {
        unsafe extern "system" fn Default<Impl: IConversationalAgentDetectorManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConversationalAgentDetectorManagerStatics, BASE_OFFSET>(),
            Default: Default::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentDetectorManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Audio", feature = "implement_exclusive"))]
pub trait IConversationalAgentSession_Impl: Sized {
    fn SessionInterrupted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSessionInterruptedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionInterrupted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SignalDetected(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSignalDetectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSignalDetected(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SystemStateChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ConversationalAgentSession, ConversationalAgentSystemStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSystemStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AgentState(&mut self) -> ::windows::core::Result<ConversationalAgentState>;
    fn Signal(&mut self) -> ::windows::core::Result<ConversationalAgentSignal>;
    fn IsIndicatorLightAvailable(&mut self) -> ::windows::core::Result<bool>;
    fn IsScreenAvailable(&mut self) -> ::windows::core::Result<bool>;
    fn IsUserAuthenticated(&mut self) -> ::windows::core::Result<bool>;
    fn IsVoiceActivationAvailable(&mut self) -> ::windows::core::Result<bool>;
    fn IsInterruptible(&mut self) -> ::windows::core::Result<bool>;
    fn IsInterrupted(&mut self) -> ::windows::core::Result<bool>;
    fn RequestInterruptibleAsync(&mut self, interruptible: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>;
    fn RequestInterruptible(&mut self, interruptible: bool) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse>;
    fn RequestAgentStateChangeAsync(&mut self, state: ConversationalAgentState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>;
    fn RequestAgentStateChange(&mut self, state: ConversationalAgentState) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse>;
    fn RequestForegroundActivationAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSessionUpdateResponse>>;
    fn RequestForegroundActivation(&mut self) -> ::windows::core::Result<ConversationalAgentSessionUpdateResponse>;
    fn GetAudioClientAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
    fn GetAudioClient(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateAudioDeviceInputNodeAsync(&mut self, graph: &::core::option::Option<super::super::Media::Audio::AudioGraph>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Media::Audio::AudioDeviceInputNode>>;
    fn CreateAudioDeviceInputNode(&mut self, graph: &::core::option::Option<super::super::Media::Audio::AudioGraph>) -> ::windows::core::Result<super::super::Media::Audio::AudioDeviceInputNode>;
    fn GetAudioCaptureDeviceIdAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetAudioCaptureDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAudioRenderDeviceIdAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetAudioRenderDeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetSignalModelIdAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn GetSignalModelId(&mut self) -> ::windows::core::Result<u32>;
    fn SetSignalModelIdAsync(&mut self, signalmodelid: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SetSignalModelId(&mut self, signalmodelid: u32) -> ::windows::core::Result<bool>;
    fn GetSupportedSignalModelIdsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>>;
    fn GetSupportedSignalModelIds(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Audio", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConversationalAgentSession {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Media_Audio", feature = "implement_exclusive"))]
impl IConversationalAgentSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSession_Vtbl {
        unsafe extern "system" fn SessionInterrupted<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSessionInterrupted<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSessionInterrupted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignalDetected<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSignalDetected<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSignalDetected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SystemStateChanged<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSystemStateChanged<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSystemStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AgentState<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Signal<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsIndicatorLightAvailable<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsScreenAvailable<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsUserAuthenticated<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsVoiceActivationAvailable<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInterruptible<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInterrupted<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestInterruptibleAsync<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interruptible: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestInterruptible<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interruptible: bool, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAgentStateChangeAsync<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: ConversationalAgentState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAgentStateChange<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: ConversationalAgentState, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestForegroundActivationAsync<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestForegroundActivation<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentSessionUpdateResponse) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAudioClientAsync<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAudioClient<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateAudioDeviceInputNodeAsync<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, graph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateAudioDeviceInputNode<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, graph: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAudioCaptureDeviceIdAsync<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAudioCaptureDeviceId<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAudioRenderDeviceIdAsync<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAudioRenderDeviceId<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSignalModelIdAsync<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSignalModelId<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSignalModelIdAsync<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalmodelid: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSignalModelId<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, signalmodelid: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSupportedSignalModelIdsAsync<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSupportedSignalModelIds<Impl: IConversationalAgentSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConversationalAgentSession, BASE_OFFSET>(),
            SessionInterrupted: SessionInterrupted::<Impl, IMPL_OFFSET>,
            RemoveSessionInterrupted: RemoveSessionInterrupted::<Impl, IMPL_OFFSET>,
            SignalDetected: SignalDetected::<Impl, IMPL_OFFSET>,
            RemoveSignalDetected: RemoveSignalDetected::<Impl, IMPL_OFFSET>,
            SystemStateChanged: SystemStateChanged::<Impl, IMPL_OFFSET>,
            RemoveSystemStateChanged: RemoveSystemStateChanged::<Impl, IMPL_OFFSET>,
            AgentState: AgentState::<Impl, IMPL_OFFSET>,
            Signal: Signal::<Impl, IMPL_OFFSET>,
            IsIndicatorLightAvailable: IsIndicatorLightAvailable::<Impl, IMPL_OFFSET>,
            IsScreenAvailable: IsScreenAvailable::<Impl, IMPL_OFFSET>,
            IsUserAuthenticated: IsUserAuthenticated::<Impl, IMPL_OFFSET>,
            IsVoiceActivationAvailable: IsVoiceActivationAvailable::<Impl, IMPL_OFFSET>,
            IsInterruptible: IsInterruptible::<Impl, IMPL_OFFSET>,
            IsInterrupted: IsInterrupted::<Impl, IMPL_OFFSET>,
            RequestInterruptibleAsync: RequestInterruptibleAsync::<Impl, IMPL_OFFSET>,
            RequestInterruptible: RequestInterruptible::<Impl, IMPL_OFFSET>,
            RequestAgentStateChangeAsync: RequestAgentStateChangeAsync::<Impl, IMPL_OFFSET>,
            RequestAgentStateChange: RequestAgentStateChange::<Impl, IMPL_OFFSET>,
            RequestForegroundActivationAsync: RequestForegroundActivationAsync::<Impl, IMPL_OFFSET>,
            RequestForegroundActivation: RequestForegroundActivation::<Impl, IMPL_OFFSET>,
            GetAudioClientAsync: GetAudioClientAsync::<Impl, IMPL_OFFSET>,
            GetAudioClient: GetAudioClient::<Impl, IMPL_OFFSET>,
            CreateAudioDeviceInputNodeAsync: CreateAudioDeviceInputNodeAsync::<Impl, IMPL_OFFSET>,
            CreateAudioDeviceInputNode: CreateAudioDeviceInputNode::<Impl, IMPL_OFFSET>,
            GetAudioCaptureDeviceIdAsync: GetAudioCaptureDeviceIdAsync::<Impl, IMPL_OFFSET>,
            GetAudioCaptureDeviceId: GetAudioCaptureDeviceId::<Impl, IMPL_OFFSET>,
            GetAudioRenderDeviceIdAsync: GetAudioRenderDeviceIdAsync::<Impl, IMPL_OFFSET>,
            GetAudioRenderDeviceId: GetAudioRenderDeviceId::<Impl, IMPL_OFFSET>,
            GetSignalModelIdAsync: GetSignalModelIdAsync::<Impl, IMPL_OFFSET>,
            GetSignalModelId: GetSignalModelId::<Impl, IMPL_OFFSET>,
            SetSignalModelIdAsync: SetSignalModelIdAsync::<Impl, IMPL_OFFSET>,
            SetSignalModelId: SetSignalModelId::<Impl, IMPL_OFFSET>,
            GetSupportedSignalModelIdsAsync: GetSupportedSignalModelIdsAsync::<Impl, IMPL_OFFSET>,
            GetSupportedSignalModelIds: GetSupportedSignalModelIds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IConversationalAgentSession2_Impl: Sized {
    fn RequestActivationAsync(&mut self, activationkind: ConversationalAgentActivationKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentActivationResult>>;
    fn RequestActivation(&mut self, activationkind: ConversationalAgentActivationKind) -> ::windows::core::Result<ConversationalAgentActivationResult>;
    fn SetSupportLockScreenActivationAsync(&mut self, lockscreenactivationsupported: bool) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetSupportLockScreenActivation(&mut self, lockscreenactivationsupported: bool) -> ::windows::core::Result<()>;
    fn GetMissingPrerequisites(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>;
    fn GetMissingPrerequisitesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<ConversationalAgentVoiceActivationPrerequisiteKind>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConversationalAgentSession2 {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSession2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IConversationalAgentSession2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSession2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSession2_Vtbl {
        unsafe extern "system" fn RequestActivationAsync<Impl: IConversationalAgentSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activationkind: ConversationalAgentActivationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestActivation<Impl: IConversationalAgentSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activationkind: ConversationalAgentActivationKind, result__: *mut ConversationalAgentActivationResult) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSupportLockScreenActivationAsync<Impl: IConversationalAgentSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lockscreenactivationsupported: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSupportLockScreenActivation<Impl: IConversationalAgentSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lockscreenactivationsupported: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSupportLockScreenActivation(lockscreenactivationsupported).into()
        }
        unsafe extern "system" fn GetMissingPrerequisites<Impl: IConversationalAgentSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMissingPrerequisitesAsync<Impl: IConversationalAgentSession2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConversationalAgentSession2, BASE_OFFSET>(),
            RequestActivationAsync: RequestActivationAsync::<Impl, IMPL_OFFSET>,
            RequestActivation: RequestActivation::<Impl, IMPL_OFFSET>,
            SetSupportLockScreenActivationAsync: SetSupportLockScreenActivationAsync::<Impl, IMPL_OFFSET>,
            SetSupportLockScreenActivation: SetSupportLockScreenActivation::<Impl, IMPL_OFFSET>,
            GetMissingPrerequisites: GetMissingPrerequisites::<Impl, IMPL_OFFSET>,
            GetMissingPrerequisitesAsync: GetMissingPrerequisitesAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSession2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSessionInterruptedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConversationalAgentSessionInterruptedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionInterruptedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IConversationalAgentSessionInterruptedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSessionInterruptedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSessionInterruptedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IConversationalAgentSessionInterruptedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSessionInterruptedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConversationalAgentSessionStatics_Impl: Sized {
    fn GetCurrentSessionAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ConversationalAgentSession>>;
    fn GetCurrentSessionSync(&mut self) -> ::windows::core::Result<ConversationalAgentSession>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConversationalAgentSessionStatics {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSessionStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConversationalAgentSessionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSessionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSessionStatics_Vtbl {
        unsafe extern "system" fn GetCurrentSessionAsync<Impl: IConversationalAgentSessionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentSessionSync<Impl: IConversationalAgentSessionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConversationalAgentSessionStatics, BASE_OFFSET>(),
            GetCurrentSessionAsync: GetCurrentSessionAsync::<Impl, IMPL_OFFSET>,
            GetCurrentSessionSync: GetCurrentSessionSync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSessionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IConversationalAgentSignal_Impl: Sized {
    fn IsSignalVerificationRequired(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsSignalVerificationRequired(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SignalId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSignalId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SignalName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSignalName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SignalContext(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSignalContext(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SignalStart(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetSignalStart(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SignalEnd(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetSignalEnd(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IConversationalAgentSignal {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IConversationalAgentSignal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSignal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSignal_Vtbl {
        unsafe extern "system" fn IsSignalVerificationRequired<Impl: IConversationalAgentSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsSignalVerificationRequired<Impl: IConversationalAgentSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSignalVerificationRequired(value).into()
        }
        unsafe extern "system" fn SignalId<Impl: IConversationalAgentSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSignalId<Impl: IConversationalAgentSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignalId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignalName<Impl: IConversationalAgentSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSignalName<Impl: IConversationalAgentSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignalName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignalContext<Impl: IConversationalAgentSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSignalContext<Impl: IConversationalAgentSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignalContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignalStart<Impl: IConversationalAgentSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSignalStart<Impl: IConversationalAgentSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignalStart(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SignalEnd<Impl: IConversationalAgentSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSignalEnd<Impl: IConversationalAgentSignal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSignalEnd(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConversationalAgentSignal, BASE_OFFSET>(),
            IsSignalVerificationRequired: IsSignalVerificationRequired::<Impl, IMPL_OFFSET>,
            SetIsSignalVerificationRequired: SetIsSignalVerificationRequired::<Impl, IMPL_OFFSET>,
            SignalId: SignalId::<Impl, IMPL_OFFSET>,
            SetSignalId: SetSignalId::<Impl, IMPL_OFFSET>,
            SignalName: SignalName::<Impl, IMPL_OFFSET>,
            SetSignalName: SetSignalName::<Impl, IMPL_OFFSET>,
            SignalContext: SignalContext::<Impl, IMPL_OFFSET>,
            SetSignalContext: SetSignalContext::<Impl, IMPL_OFFSET>,
            SignalStart: SignalStart::<Impl, IMPL_OFFSET>,
            SetSignalStart: SetSignalStart::<Impl, IMPL_OFFSET>,
            SignalEnd: SignalEnd::<Impl, IMPL_OFFSET>,
            SetSignalEnd: SetSignalEnd::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSignal as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSignal2_Impl: Sized {
    fn DetectorId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DetectorKind(&mut self) -> ::windows::core::Result<ActivationSignalDetectorKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConversationalAgentSignal2 {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignal2";
}
#[cfg(feature = "implement_exclusive")]
impl IConversationalAgentSignal2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSignal2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSignal2_Vtbl {
        unsafe extern "system" fn DetectorId<Impl: IConversationalAgentSignal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DetectorKind<Impl: IConversationalAgentSignal2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ActivationSignalDetectorKind) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConversationalAgentSignal2, BASE_OFFSET>(),
            DetectorId: DetectorId::<Impl, IMPL_OFFSET>,
            DetectorKind: DetectorKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSignal2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSignalDetectedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConversationalAgentSignalDetectedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSignalDetectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IConversationalAgentSignalDetectedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSignalDetectedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSignalDetectedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IConversationalAgentSignalDetectedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSignalDetectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IConversationalAgentSystemStateChangedEventArgs_Impl: Sized {
    fn SystemStateChangeType(&mut self) -> ::windows::core::Result<ConversationalAgentSystemStateChangeType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IConversationalAgentSystemStateChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IConversationalAgentSystemStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IConversationalAgentSystemStateChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConversationalAgentSystemStateChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConversationalAgentSystemStateChangedEventArgs_Vtbl {
        unsafe extern "system" fn SystemStateChangeType<Impl: IConversationalAgentSystemStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ConversationalAgentSystemStateChangeType) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IConversationalAgentSystemStateChangedEventArgs, BASE_OFFSET>(),
            SystemStateChangeType: SystemStateChangeType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConversationalAgentSystemStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDetectionConfigurationAvailabilityChangedEventArgs_Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<DetectionConfigurationAvailabilityChangeKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDetectionConfigurationAvailabilityChangedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDetectionConfigurationAvailabilityChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDetectionConfigurationAvailabilityChangedEventArgs_Vtbl {
        unsafe extern "system" fn Kind<Impl: IDetectionConfigurationAvailabilityChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DetectionConfigurationAvailabilityChangeKind) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDetectionConfigurationAvailabilityChangedEventArgs, BASE_OFFSET>(),
            Kind: Kind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDetectionConfigurationAvailabilityChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDetectionConfigurationAvailabilityInfo_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn HasSystemResourceAccess(&mut self) -> ::windows::core::Result<bool>;
    fn HasPermission(&mut self) -> ::windows::core::Result<bool>;
    fn HasLockScreenPermission(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDetectionConfigurationAvailabilityInfo {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IDetectionConfigurationAvailabilityInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDetectionConfigurationAvailabilityInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDetectionConfigurationAvailabilityInfo_Vtbl {
        unsafe extern "system" fn IsEnabled<Impl: IDetectionConfigurationAvailabilityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasSystemResourceAccess<Impl: IDetectionConfigurationAvailabilityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasPermission<Impl: IDetectionConfigurationAvailabilityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasLockScreenPermission<Impl: IDetectionConfigurationAvailabilityInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDetectionConfigurationAvailabilityInfo, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            HasSystemResourceAccess: HasSystemResourceAccess::<Impl, IMPL_OFFSET>,
            HasPermission: HasPermission::<Impl, IMPL_OFFSET>,
            HasLockScreenPermission: HasLockScreenPermission::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDetectionConfigurationAvailabilityInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDetectionConfigurationAvailabilityInfo2_Impl: Sized {
    fn UnavailableSystemResources(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SignalDetectorResourceKind>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDetectionConfigurationAvailabilityInfo2 {
    const NAME: &'static str = "Windows.ApplicationModel.ConversationalAgent.IDetectionConfigurationAvailabilityInfo2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDetectionConfigurationAvailabilityInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDetectionConfigurationAvailabilityInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDetectionConfigurationAvailabilityInfo2_Vtbl {
        unsafe extern "system" fn UnavailableSystemResources<Impl: IDetectionConfigurationAvailabilityInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDetectionConfigurationAvailabilityInfo2, BASE_OFFSET>(),
            UnavailableSystemResources: UnavailableSystemResources::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDetectionConfigurationAvailabilityInfo2 as ::windows::core::Interface>::IID
    }
}
