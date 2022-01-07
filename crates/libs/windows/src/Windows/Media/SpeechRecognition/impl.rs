#[cfg(feature = "implement_exclusive")]
pub trait ISpeechContinuousRecognitionCompletedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SpeechRecognitionResultStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechContinuousRecognitionCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechContinuousRecognitionCompletedEventArgsVtbl {
    pub const fn new<Impl: ISpeechContinuousRecognitionCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechContinuousRecognitionCompletedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: ISpeechContinuousRecognitionCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionResultStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechContinuousRecognitionCompletedEventArgs>, base.5, Status::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechContinuousRecognitionResultGeneratedEventArgsImpl: Sized {
    fn Result(&self) -> ::windows::core::Result<SpeechRecognitionResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechContinuousRecognitionResultGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionResultGeneratedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechContinuousRecognitionResultGeneratedEventArgsVtbl {
    pub const fn new<Impl: ISpeechContinuousRecognitionResultGeneratedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechContinuousRecognitionResultGeneratedEventArgsVtbl {
        unsafe extern "system" fn Result<Impl: ISpeechContinuousRecognitionResultGeneratedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Result() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechContinuousRecognitionResultGeneratedEventArgs>, base.5, Result::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechContinuousRecognitionSessionImpl: Sized {
    fn AutoStopSilenceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetAutoStopSilenceTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StartWithModeAsync(&self, mode: SpeechContinuousRecognitionMode) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn CancelAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn PauseAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn Completed(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResultGenerated(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionResultGeneratedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResultGenerated(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechContinuousRecognitionSession {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechContinuousRecognitionSession";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechContinuousRecognitionSessionVtbl {
    pub const fn new<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechContinuousRecognitionSessionVtbl {
        unsafe extern "system" fn AutoStopSilenceTimeout<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AutoStopSilenceTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoStopSilenceTimeout<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAutoStopSilenceTimeout(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartAsync<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartWithModeAsync<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: SpeechContinuousRecognitionMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartWithModeAsync(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopAsync<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAsync<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PauseAsync<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PauseAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn Completed<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&value as *const <super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&value as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResultGenerated<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResultGenerated(&*(&value as *const <super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionResultGeneratedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpeechContinuousRecognitionSession, SpeechContinuousRecognitionResultGeneratedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResultGenerated<Impl: ISpeechContinuousRecognitionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveResultGenerated(&*(&value as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISpeechContinuousRecognitionSession>,
            base.5,
            AutoStopSilenceTimeout::<Impl, OFFSET>,
            SetAutoStopSilenceTimeout::<Impl, OFFSET>,
            StartAsync::<Impl, OFFSET>,
            StartWithModeAsync::<Impl, OFFSET>,
            StopAsync::<Impl, OFFSET>,
            CancelAsync::<Impl, OFFSET>,
            PauseAsync::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            Completed::<Impl, OFFSET>,
            RemoveCompleted::<Impl, OFFSET>,
            ResultGenerated::<Impl, OFFSET>,
            RemoveResultGenerated::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionCompilationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SpeechRecognitionResultStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionCompilationResult {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionCompilationResult";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionCompilationResultVtbl {
    pub const fn new<Impl: ISpeechRecognitionCompilationResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionCompilationResultVtbl {
        unsafe extern "system" fn Status<Impl: ISpeechRecognitionCompilationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionResultStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionCompilationResult>, base.5, Status::<Impl, OFFSET>)
    }
}
pub trait ISpeechRecognitionConstraintImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTag(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<SpeechRecognitionConstraintType>;
    fn Probability(&self) -> ::windows::core::Result<SpeechRecognitionConstraintProbability>;
    fn SetProbability(&self, value: SpeechRecognitionConstraintProbability) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISpeechRecognitionConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionConstraint";
}
impl ISpeechRecognitionConstraintVtbl {
    pub const fn new<Impl: ISpeechRecognitionConstraintImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionConstraintVtbl {
        unsafe extern "system" fn IsEnabled<Impl: ISpeechRecognitionConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: ISpeechRecognitionConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn Tag<Impl: ISpeechRecognitionConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: ISpeechRecognitionConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Type<Impl: ISpeechRecognitionConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Probability<Impl: ISpeechRecognitionConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConstraintProbability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Probability() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProbability<Impl: ISpeechRecognitionConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SpeechRecognitionConstraintProbability) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProbability(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionConstraint>, base.5, IsEnabled::<Impl, OFFSET>, SetIsEnabled::<Impl, OFFSET>, Tag::<Impl, OFFSET>, SetTag::<Impl, OFFSET>, Type::<Impl, OFFSET>, Probability::<Impl, OFFSET>, SetProbability::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionGrammarFileConstraintImpl: Sized + ISpeechRecognitionConstraintImpl {
    fn GrammarFile(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionGrammarFileConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraint";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionGrammarFileConstraintVtbl {
    pub const fn new<Impl: ISpeechRecognitionGrammarFileConstraintImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionGrammarFileConstraintVtbl {
        unsafe extern "system" fn GrammarFile<Impl: ISpeechRecognitionGrammarFileConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GrammarFile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionGrammarFileConstraint>, base.5, GrammarFile::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionGrammarFileConstraintFactoryImpl: Sized {
    fn Create(&self, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<SpeechRecognitionGrammarFileConstraint>;
    fn CreateWithTag(&self, file: &::core::option::Option<super::super::Storage::StorageFile>, tag: &::windows::core::HSTRING) -> ::windows::core::Result<SpeechRecognitionGrammarFileConstraint>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionGrammarFileConstraintFactory {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionGrammarFileConstraintFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionGrammarFileConstraintFactoryVtbl {
    pub const fn new<Impl: ISpeechRecognitionGrammarFileConstraintFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionGrammarFileConstraintFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISpeechRecognitionGrammarFileConstraintFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&file as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithTag<Impl: ISpeechRecognitionGrammarFileConstraintFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithTag(&*(&file as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType), &*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionGrammarFileConstraintFactory>, base.5, Create::<Impl, OFFSET>, CreateWithTag::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionHypothesisImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionHypothesis {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesis";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionHypothesisVtbl {
    pub const fn new<Impl: ISpeechRecognitionHypothesisImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionHypothesisVtbl {
        unsafe extern "system" fn Text<Impl: ISpeechRecognitionHypothesisImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionHypothesis>, base.5, Text::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionHypothesisGeneratedEventArgsImpl: Sized {
    fn Hypothesis(&self) -> ::windows::core::Result<SpeechRecognitionHypothesis>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionHypothesisGeneratedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionHypothesisGeneratedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionHypothesisGeneratedEventArgsVtbl {
    pub const fn new<Impl: ISpeechRecognitionHypothesisGeneratedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionHypothesisGeneratedEventArgsVtbl {
        unsafe extern "system" fn Hypothesis<Impl: ISpeechRecognitionHypothesisGeneratedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Hypothesis() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionHypothesisGeneratedEventArgs>, base.5, Hypothesis::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionListConstraintImpl: Sized + ISpeechRecognitionConstraintImpl {
    fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionListConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraint";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionListConstraintVtbl {
    pub const fn new<Impl: ISpeechRecognitionListConstraintImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionListConstraintVtbl {
        unsafe extern "system" fn Commands<Impl: ISpeechRecognitionListConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Commands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionListConstraint>, base.5, Commands::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionListConstraintFactoryImpl: Sized {
    fn Create(&self, commands: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<SpeechRecognitionListConstraint>;
    fn CreateWithTag(&self, commands: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, tag: &::windows::core::HSTRING) -> ::windows::core::Result<SpeechRecognitionListConstraint>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionListConstraintFactory {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionListConstraintFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionListConstraintFactoryVtbl {
    pub const fn new<Impl: ISpeechRecognitionListConstraintFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionListConstraintFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISpeechRecognitionListConstraintFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commands: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&commands as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithTag<Impl: ISpeechRecognitionListConstraintFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commands: ::windows::core::RawPtr, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithTag(&*(&commands as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType), &*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionListConstraintFactory>, base.5, Create::<Impl, OFFSET>, CreateWithTag::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionQualityDegradingEventArgsImpl: Sized {
    fn Problem(&self) -> ::windows::core::Result<SpeechRecognitionAudioProblem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionQualityDegradingEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionQualityDegradingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionQualityDegradingEventArgsVtbl {
    pub const fn new<Impl: ISpeechRecognitionQualityDegradingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionQualityDegradingEventArgsVtbl {
        unsafe extern "system" fn Problem<Impl: ISpeechRecognitionQualityDegradingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionAudioProblem) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Problem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionQualityDegradingEventArgs>, base.5, Problem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SpeechRecognitionResultStatus>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Confidence(&self) -> ::windows::core::Result<SpeechRecognitionConfidence>;
    fn SemanticInterpretation(&self) -> ::windows::core::Result<SpeechRecognitionSemanticInterpretation>;
    fn GetAlternates(&self, maxalternates: u32) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SpeechRecognitionResult>>;
    fn Constraint(&self) -> ::windows::core::Result<ISpeechRecognitionConstraint>;
    fn RulePath(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn RawConfidence(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionResult {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionResult";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionResultVtbl {
    pub const fn new<Impl: ISpeechRecognitionResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionResultVtbl {
        unsafe extern "system" fn Status<Impl: ISpeechRecognitionResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionResultStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: ISpeechRecognitionResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Confidence<Impl: ISpeechRecognitionResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionConfidence) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Confidence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SemanticInterpretation<Impl: ISpeechRecognitionResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SemanticInterpretation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlternates<Impl: ISpeechRecognitionResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxalternates: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAlternates(maxalternates) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Constraint<Impl: ISpeechRecognitionResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Constraint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RulePath<Impl: ISpeechRecognitionResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RulePath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawConfidence<Impl: ISpeechRecognitionResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RawConfidence() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionResult>, base.5, Status::<Impl, OFFSET>, Text::<Impl, OFFSET>, Confidence::<Impl, OFFSET>, SemanticInterpretation::<Impl, OFFSET>, GetAlternates::<Impl, OFFSET>, Constraint::<Impl, OFFSET>, RulePath::<Impl, OFFSET>, RawConfidence::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionResult2Impl: Sized {
    fn PhraseStartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn PhraseDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionResult2 {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionResult2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionResult2Vtbl {
    pub const fn new<Impl: ISpeechRecognitionResult2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionResult2Vtbl {
        unsafe extern "system" fn PhraseStartTime<Impl: ISpeechRecognitionResult2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhraseStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhraseDuration<Impl: ISpeechRecognitionResult2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PhraseDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionResult2>, base.5, PhraseStartTime::<Impl, OFFSET>, PhraseDuration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionSemanticInterpretationImpl: Sized {
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionSemanticInterpretation {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionSemanticInterpretation";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionSemanticInterpretationVtbl {
    pub const fn new<Impl: ISpeechRecognitionSemanticInterpretationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionSemanticInterpretationVtbl {
        unsafe extern "system" fn Properties<Impl: ISpeechRecognitionSemanticInterpretationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionSemanticInterpretation>, base.5, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionTopicConstraintImpl: Sized + ISpeechRecognitionConstraintImpl {
    fn Scenario(&self) -> ::windows::core::Result<SpeechRecognitionScenario>;
    fn TopicHint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionTopicConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraint";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionTopicConstraintVtbl {
    pub const fn new<Impl: ISpeechRecognitionTopicConstraintImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionTopicConstraintVtbl {
        unsafe extern "system" fn Scenario<Impl: ISpeechRecognitionTopicConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognitionScenario) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Scenario() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TopicHint<Impl: ISpeechRecognitionTopicConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TopicHint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionTopicConstraint>, base.5, Scenario::<Impl, OFFSET>, TopicHint::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionTopicConstraintFactoryImpl: Sized {
    fn Create(&self, scenario: SpeechRecognitionScenario, topichint: &::windows::core::HSTRING) -> ::windows::core::Result<SpeechRecognitionTopicConstraint>;
    fn CreateWithTag(&self, scenario: SpeechRecognitionScenario, topichint: &::windows::core::HSTRING, tag: &::windows::core::HSTRING) -> ::windows::core::Result<SpeechRecognitionTopicConstraint>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionTopicConstraintFactory {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionTopicConstraintFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionTopicConstraintFactoryVtbl {
    pub const fn new<Impl: ISpeechRecognitionTopicConstraintFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionTopicConstraintFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISpeechRecognitionTopicConstraintFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scenario: SpeechRecognitionScenario, topichint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(scenario, &*(&topichint as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithTag<Impl: ISpeechRecognitionTopicConstraintFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scenario: SpeechRecognitionScenario, topichint: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, tag: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithTag(scenario, &*(&topichint as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&tag as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionTopicConstraintFactory>, base.5, Create::<Impl, OFFSET>, CreateWithTag::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognitionVoiceCommandDefinitionConstraintImpl: Sized + ISpeechRecognitionConstraintImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognitionVoiceCommandDefinitionConstraint {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognitionVoiceCommandDefinitionConstraint";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognitionVoiceCommandDefinitionConstraintVtbl {
    pub const fn new<Impl: ISpeechRecognitionVoiceCommandDefinitionConstraintImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognitionVoiceCommandDefinitionConstraintVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognitionVoiceCommandDefinitionConstraint>, base.5)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpeechRecognizerImpl: Sized + IClosableImpl {
    fn CurrentLanguage(&self) -> ::windows::core::Result<super::super::Globalization::Language>;
    fn Constraints(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISpeechRecognitionConstraint>>;
    fn Timeouts(&self) -> ::windows::core::Result<SpeechRecognizerTimeouts>;
    fn UIOptions(&self) -> ::windows::core::Result<SpeechRecognizerUIOptions>;
    fn CompileConstraintsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionCompilationResult>>;
    fn RecognizeAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>>;
    fn RecognizeWithUIAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpeechRecognitionResult>>;
    fn RecognitionQualityDegrading(&self, speechrecognitionqualitydegradinghandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionQualityDegradingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRecognitionQualityDegrading(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StateChanged(&self, statechangedhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognizerStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISpeechRecognizer {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognizer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISpeechRecognizerVtbl {
    pub const fn new<Impl: ISpeechRecognizerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognizerVtbl {
        unsafe extern "system" fn CurrentLanguage<Impl: ISpeechRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Constraints<Impl: ISpeechRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Constraints() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timeouts<Impl: ISpeechRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timeouts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UIOptions<Impl: ISpeechRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UIOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompileConstraintsAsync<Impl: ISpeechRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompileConstraintsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognizeAsync<Impl: ISpeechRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognizeAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognizeWithUIAsync<Impl: ISpeechRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognizeWithUIAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecognitionQualityDegrading<Impl: ISpeechRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, speechrecognitionqualitydegradinghandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RecognitionQualityDegrading(&*(&speechrecognitionqualitydegradinghandler as *const <super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionQualityDegradingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionQualityDegradingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRecognitionQualityDegrading<Impl: ISpeechRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRecognitionQualityDegrading(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StateChanged<Impl: ISpeechRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statechangedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StateChanged(&*(&statechangedhandler as *const <super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognizerStateChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognizerStateChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStateChanged<Impl: ISpeechRecognizerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveStateChanged(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISpeechRecognizer>,
            base.5,
            CurrentLanguage::<Impl, OFFSET>,
            Constraints::<Impl, OFFSET>,
            Timeouts::<Impl, OFFSET>,
            UIOptions::<Impl, OFFSET>,
            CompileConstraintsAsync::<Impl, OFFSET>,
            RecognizeAsync::<Impl, OFFSET>,
            RecognizeWithUIAsync::<Impl, OFFSET>,
            RecognitionQualityDegrading::<Impl, OFFSET>,
            RemoveRecognitionQualityDegrading::<Impl, OFFSET>,
            StateChanged::<Impl, OFFSET>,
            RemoveStateChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizer2Impl: Sized {
    fn ContinuousRecognitionSession(&self) -> ::windows::core::Result<SpeechContinuousRecognitionSession>;
    fn State(&self) -> ::windows::core::Result<SpeechRecognizerState>;
    fn StopRecognitionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn HypothesisGenerated(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionHypothesisGeneratedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHypothesisGenerated(&self, value: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognizer2 {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognizer2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognizer2Vtbl {
    pub const fn new<Impl: ISpeechRecognizer2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognizer2Vtbl {
        unsafe extern "system" fn ContinuousRecognitionSession<Impl: ISpeechRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContinuousRecognitionSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn State<Impl: ISpeechRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognizerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopRecognitionAsync<Impl: ISpeechRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StopRecognitionAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HypothesisGenerated<Impl: ISpeechRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HypothesisGenerated(&*(&value as *const <super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionHypothesisGeneratedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SpeechRecognizer, SpeechRecognitionHypothesisGeneratedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHypothesisGenerated<Impl: ISpeechRecognizer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveHypothesisGenerated(&*(&value as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognizer2>, base.5, ContinuousRecognitionSession::<Impl, OFFSET>, State::<Impl, OFFSET>, StopRecognitionAsync::<Impl, OFFSET>, HypothesisGenerated::<Impl, OFFSET>, RemoveHypothesisGenerated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizerFactoryImpl: Sized {
    fn Create(&self, language: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<SpeechRecognizer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognizerFactory {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognizerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognizerFactoryVtbl {
    pub const fn new<Impl: ISpeechRecognizerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognizerFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISpeechRecognizerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&language as *const <super::super::Globalization::Language as ::windows::core::Abi>::Abi as *const <super::super::Globalization::Language as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognizerFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizerStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<SpeechRecognizerState>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognizerStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognizerStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognizerStateChangedEventArgsVtbl {
    pub const fn new<Impl: ISpeechRecognizerStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognizerStateChangedEventArgsVtbl {
        unsafe extern "system" fn State<Impl: ISpeechRecognizerStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SpeechRecognizerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognizerStateChangedEventArgs>, base.5, State::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizerStaticsImpl: Sized {
    fn SystemSpeechLanguage(&self) -> ::windows::core::Result<super::super::Globalization::Language>;
    fn SupportedTopicLanguages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>;
    fn SupportedGrammarLanguages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognizerStatics {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognizerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognizerStaticsVtbl {
    pub const fn new<Impl: ISpeechRecognizerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognizerStaticsVtbl {
        unsafe extern "system" fn SystemSpeechLanguage<Impl: ISpeechRecognizerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemSpeechLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedTopicLanguages<Impl: ISpeechRecognizerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedTopicLanguages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedGrammarLanguages<Impl: ISpeechRecognizerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SupportedGrammarLanguages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognizerStatics>, base.5, SystemSpeechLanguage::<Impl, OFFSET>, SupportedTopicLanguages::<Impl, OFFSET>, SupportedGrammarLanguages::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizerStatics2Impl: Sized {
    fn TrySetSystemSpeechLanguageAsync(&self, speechlanguage: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognizerStatics2 {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognizerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognizerStatics2Vtbl {
    pub const fn new<Impl: ISpeechRecognizerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognizerStatics2Vtbl {
        unsafe extern "system" fn TrySetSystemSpeechLanguageAsync<Impl: ISpeechRecognizerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, speechlanguage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetSystemSpeechLanguageAsync(&*(&speechlanguage as *const <super::super::Globalization::Language as ::windows::core::Abi>::Abi as *const <super::super::Globalization::Language as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognizerStatics2>, base.5, TrySetSystemSpeechLanguageAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizerTimeoutsImpl: Sized {
    fn InitialSilenceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetInitialSilenceTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn EndSilenceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetEndSilenceTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BabbleTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBabbleTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognizerTimeouts {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognizerTimeouts";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognizerTimeoutsVtbl {
    pub const fn new<Impl: ISpeechRecognizerTimeoutsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognizerTimeoutsVtbl {
        unsafe extern "system" fn InitialSilenceTimeout<Impl: ISpeechRecognizerTimeoutsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitialSilenceTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInitialSilenceTimeout<Impl: ISpeechRecognizerTimeoutsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInitialSilenceTimeout(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EndSilenceTimeout<Impl: ISpeechRecognizerTimeoutsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndSilenceTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndSilenceTimeout<Impl: ISpeechRecognizerTimeoutsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEndSilenceTimeout(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BabbleTimeout<Impl: ISpeechRecognizerTimeoutsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BabbleTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBabbleTimeout<Impl: ISpeechRecognizerTimeoutsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBabbleTimeout(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognizerTimeouts>, base.5, InitialSilenceTimeout::<Impl, OFFSET>, SetInitialSilenceTimeout::<Impl, OFFSET>, EndSilenceTimeout::<Impl, OFFSET>, SetEndSilenceTimeout::<Impl, OFFSET>, BabbleTimeout::<Impl, OFFSET>, SetBabbleTimeout::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpeechRecognizerUIOptionsImpl: Sized {
    fn ExampleText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExampleText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AudiblePrompt(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAudiblePrompt(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsReadBackEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsReadBackEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ShowConfirmation(&self) -> ::windows::core::Result<bool>;
    fn SetShowConfirmation(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISpeechRecognizerUIOptions {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.ISpeechRecognizerUIOptions";
}
#[cfg(feature = "implement_exclusive")]
impl ISpeechRecognizerUIOptionsVtbl {
    pub const fn new<Impl: ISpeechRecognizerUIOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpeechRecognizerUIOptionsVtbl {
        unsafe extern "system" fn ExampleText<Impl: ISpeechRecognizerUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExampleText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExampleText<Impl: ISpeechRecognizerUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExampleText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AudiblePrompt<Impl: ISpeechRecognizerUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AudiblePrompt() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudiblePrompt<Impl: ISpeechRecognizerUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAudiblePrompt(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsReadBackEnabled<Impl: ISpeechRecognizerUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReadBackEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsReadBackEnabled<Impl: ISpeechRecognizerUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsReadBackEnabled(value).into()
        }
        unsafe extern "system" fn ShowConfirmation<Impl: ISpeechRecognizerUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowConfirmation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowConfirmation<Impl: ISpeechRecognizerUIOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShowConfirmation(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpeechRecognizerUIOptions>, base.5, ExampleText::<Impl, OFFSET>, SetExampleText::<Impl, OFFSET>, AudiblePrompt::<Impl, OFFSET>, SetAudiblePrompt::<Impl, OFFSET>, IsReadBackEnabled::<Impl, OFFSET>, SetIsReadBackEnabled::<Impl, OFFSET>, ShowConfirmation::<Impl, OFFSET>, SetShowConfirmation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandManagerImpl: Sized {
    fn InstallCommandSetsFromStorageFileAsync(&self, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn InstalledCommandSets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandSet>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandManager {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.IVoiceCommandManager";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandManagerVtbl {
    pub const fn new<Impl: IVoiceCommandManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandManagerVtbl {
        unsafe extern "system" fn InstallCommandSetsFromStorageFileAsync<Impl: IVoiceCommandManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstallCommandSetsFromStorageFileAsync(&*(&file as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstalledCommandSets<Impl: IVoiceCommandManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstalledCommandSets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommandManager>, base.5, InstallCommandSetsFromStorageFileAsync::<Impl, OFFSET>, InstalledCommandSets::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandSetImpl: Sized {
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPhraseListAsync(&self, phraselistname: &::windows::core::HSTRING, phraselist: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandSet {
    const NAME: &'static str = "Windows.Media.SpeechRecognition.IVoiceCommandSet";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandSetVtbl {
    pub const fn new<Impl: IVoiceCommandSetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandSetVtbl {
        unsafe extern "system" fn Language<Impl: IVoiceCommandSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IVoiceCommandSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhraseListAsync<Impl: IVoiceCommandSetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phraselistname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phraselist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPhraseListAsync(&*(&phraselistname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&phraselist as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommandSet>, base.5, Language::<Impl, OFFSET>, Name::<Impl, OFFSET>, SetPhraseListAsync::<Impl, OFFSET>)
    }
}
