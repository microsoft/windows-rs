#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandImpl: Sized {
    fn CommandName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn SpeechRecognitionResult(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommand {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommand";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandVtbl {
    pub const fn new<Impl: IVoiceCommandImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandVtbl {
        unsafe extern "system" fn CommandName<Impl: IVoiceCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommandName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IVoiceCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SpeechRecognitionResult<Impl: IVoiceCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpeechRecognitionResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommand>, base.5, CommandName::<Impl, OFFSET>, Properties::<Impl, OFFSET>, SpeechRecognitionResult::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandCompletedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<VoiceCommandCompletionReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandCompletedEventArgsVtbl {
    pub const fn new<Impl: IVoiceCommandCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandCompletedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IVoiceCommandCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VoiceCommandCompletionReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommandCompletedEventArgs>, base.5, Reason::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandConfirmationResultImpl: Sized {
    fn Confirmed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandConfirmationResult {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandConfirmationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandConfirmationResultVtbl {
    pub const fn new<Impl: IVoiceCommandConfirmationResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandConfirmationResultVtbl {
        unsafe extern "system" fn Confirmed<Impl: IVoiceCommandConfirmationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Confirmed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommandConfirmationResult>, base.5, Confirmed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandContentTileImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TextLine1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTextLine1(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TextLine2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTextLine2(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TextLine3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTextLine3(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Image(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn SetImage(&self, value: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<()>;
    fn AppContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetAppContext(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AppLaunchArgument(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAppLaunchArgument(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentTileType(&self) -> ::windows::core::Result<VoiceCommandContentTileType>;
    fn SetContentTileType(&self, value: VoiceCommandContentTileType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandContentTile {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandContentTile";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandContentTileVtbl {
    pub const fn new<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandContentTileVtbl {
        unsafe extern "system" fn Title<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextLine1<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TextLine1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextLine1<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTextLine1(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextLine2<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TextLine2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextLine2<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTextLine2(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextLine3<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TextLine3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextLine3<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTextLine3(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Image<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&value as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppContext<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppContext<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAppContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppLaunchArgument<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppLaunchArgument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppLaunchArgument<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAppLaunchArgument(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTileType<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VoiceCommandContentTileType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTileType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTileType<Impl: IVoiceCommandContentTileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VoiceCommandContentTileType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentTileType(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IVoiceCommandContentTile>,
            base.5,
            Title::<Impl, OFFSET>,
            SetTitle::<Impl, OFFSET>,
            TextLine1::<Impl, OFFSET>,
            SetTextLine1::<Impl, OFFSET>,
            TextLine2::<Impl, OFFSET>,
            SetTextLine2::<Impl, OFFSET>,
            TextLine3::<Impl, OFFSET>,
            SetTextLine3::<Impl, OFFSET>,
            Image::<Impl, OFFSET>,
            SetImage::<Impl, OFFSET>,
            AppContext::<Impl, OFFSET>,
            SetAppContext::<Impl, OFFSET>,
            AppLaunchArgument::<Impl, OFFSET>,
            SetAppLaunchArgument::<Impl, OFFSET>,
            ContentTileType::<Impl, OFFSET>,
            SetContentTileType::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandDefinitionImpl: Sized {
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPhraseListAsync(&self, phraselistname: &::windows::core::HSTRING, phraselist: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandDefinition {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandDefinition";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandDefinitionVtbl {
    pub const fn new<Impl: IVoiceCommandDefinitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandDefinitionVtbl {
        unsafe extern "system" fn Language<Impl: IVoiceCommandDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Name<Impl: IVoiceCommandDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPhraseListAsync<Impl: IVoiceCommandDefinitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phraselistname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phraselist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommandDefinition>, base.5, Language::<Impl, OFFSET>, Name::<Impl, OFFSET>, SetPhraseListAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandDefinitionManagerStaticsImpl: Sized {
    fn InstallCommandDefinitionsFromStorageFileAsync(&self, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn InstalledCommandDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandDefinition>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandDefinitionManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandDefinitionManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandDefinitionManagerStaticsVtbl {
    pub const fn new<Impl: IVoiceCommandDefinitionManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandDefinitionManagerStaticsVtbl {
        unsafe extern "system" fn InstallCommandDefinitionsFromStorageFileAsync<Impl: IVoiceCommandDefinitionManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstallCommandDefinitionsFromStorageFileAsync(&*(&file as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstalledCommandDefinitions<Impl: IVoiceCommandDefinitionManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InstalledCommandDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommandDefinitionManagerStatics>, base.5, InstallCommandDefinitionsFromStorageFileAsync::<Impl, OFFSET>, InstalledCommandDefinitions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandDisambiguationResultImpl: Sized {
    fn SelectedItem(&self) -> ::windows::core::Result<VoiceCommandContentTile>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandDisambiguationResult {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandDisambiguationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandDisambiguationResultVtbl {
    pub const fn new<Impl: IVoiceCommandDisambiguationResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandDisambiguationResultVtbl {
        unsafe extern "system" fn SelectedItem<Impl: IVoiceCommandDisambiguationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommandDisambiguationResult>, base.5, SelectedItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandResponseImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<VoiceCommandUserMessage>;
    fn SetMessage(&self, value: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<()>;
    fn RepeatMessage(&self) -> ::windows::core::Result<VoiceCommandUserMessage>;
    fn SetRepeatMessage(&self, value: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<()>;
    fn AppLaunchArgument(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAppLaunchArgument(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn VoiceCommandContentTiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VoiceCommandContentTile>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandResponse {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandResponse";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandResponseVtbl {
    pub const fn new<Impl: IVoiceCommandResponseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandResponseVtbl {
        unsafe extern "system" fn Message<Impl: IVoiceCommandResponseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessage<Impl: IVoiceCommandResponseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMessage(&*(&value as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RepeatMessage<Impl: IVoiceCommandResponseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RepeatMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepeatMessage<Impl: IVoiceCommandResponseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRepeatMessage(&*(&value as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppLaunchArgument<Impl: IVoiceCommandResponseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppLaunchArgument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppLaunchArgument<Impl: IVoiceCommandResponseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAppLaunchArgument(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VoiceCommandContentTiles<Impl: IVoiceCommandResponseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VoiceCommandContentTiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommandResponse>, base.5, Message::<Impl, OFFSET>, SetMessage::<Impl, OFFSET>, RepeatMessage::<Impl, OFFSET>, SetRepeatMessage::<Impl, OFFSET>, AppLaunchArgument::<Impl, OFFSET>, SetAppLaunchArgument::<Impl, OFFSET>, VoiceCommandContentTiles::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandResponseStaticsImpl: Sized {
    fn MaxSupportedVoiceCommandContentTiles(&self) -> ::windows::core::Result<u32>;
    fn CreateResponse(&self, usermessage: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<VoiceCommandResponse>;
    fn CreateResponseWithTiles(&self, message: &::core::option::Option<VoiceCommandUserMessage>, contenttiles: &::core::option::Option<super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>) -> ::windows::core::Result<VoiceCommandResponse>;
    fn CreateResponseForPrompt(&self, message: &::core::option::Option<VoiceCommandUserMessage>, repeatmessage: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<VoiceCommandResponse>;
    fn CreateResponseForPromptWithTiles(&self, message: &::core::option::Option<VoiceCommandUserMessage>, repeatmessage: &::core::option::Option<VoiceCommandUserMessage>, contenttiles: &::core::option::Option<super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>) -> ::windows::core::Result<VoiceCommandResponse>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandResponseStatics {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandResponseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandResponseStaticsVtbl {
    pub const fn new<Impl: IVoiceCommandResponseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandResponseStaticsVtbl {
        unsafe extern "system" fn MaxSupportedVoiceCommandContentTiles<Impl: IVoiceCommandResponseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxSupportedVoiceCommandContentTiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResponse<Impl: IVoiceCommandResponseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usermessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateResponse(&*(&usermessage as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResponseWithTiles<Impl: IVoiceCommandResponseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, contenttiles: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateResponseWithTiles(&*(&message as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType), &*(&contenttiles as *const <super::super::Foundation::Collections::IIterable<VoiceCommandContentTile> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<VoiceCommandContentTile> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResponseForPrompt<Impl: IVoiceCommandResponseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, repeatmessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateResponseForPrompt(&*(&message as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType), &*(&repeatmessage as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResponseForPromptWithTiles<Impl: IVoiceCommandResponseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, repeatmessage: ::windows::core::RawPtr, contenttiles: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateResponseForPromptWithTiles(
                &*(&message as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType),
                &*(&repeatmessage as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType),
                &*(&contenttiles as *const <super::super::Foundation::Collections::IIterable<VoiceCommandContentTile> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<VoiceCommandContentTile> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommandResponseStatics>, base.5, MaxSupportedVoiceCommandContentTiles::<Impl, OFFSET>, CreateResponse::<Impl, OFFSET>, CreateResponseWithTiles::<Impl, OFFSET>, CreateResponseForPrompt::<Impl, OFFSET>, CreateResponseForPromptWithTiles::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandServiceConnectionImpl: Sized {
    fn GetVoiceCommandAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoiceCommand>>;
    fn RequestConfirmationAsync(&self, response: &::core::option::Option<VoiceCommandResponse>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoiceCommandConfirmationResult>>;
    fn RequestDisambiguationAsync(&self, response: &::core::option::Option<VoiceCommandResponse>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoiceCommandDisambiguationResult>>;
    fn ReportProgressAsync(&self, response: &::core::option::Option<VoiceCommandResponse>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ReportSuccessAsync(&self, response: &::core::option::Option<VoiceCommandResponse>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ReportFailureAsync(&self, response: &::core::option::Option<VoiceCommandResponse>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestAppLaunchAsync(&self, response: &::core::option::Option<VoiceCommandResponse>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Language(&self) -> ::windows::core::Result<super::super::Globalization::Language>;
    fn VoiceCommandCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoiceCommandServiceConnection, VoiceCommandCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVoiceCommandCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandServiceConnection";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandServiceConnectionVtbl {
    pub const fn new<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandServiceConnectionVtbl {
        unsafe extern "system" fn GetVoiceCommandAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVoiceCommandAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestConfirmationAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestConfirmationAsync(&*(&response as *const <VoiceCommandResponse as ::windows::core::Abi>::Abi as *const <VoiceCommandResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestDisambiguationAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestDisambiguationAsync(&*(&response as *const <VoiceCommandResponse as ::windows::core::Abi>::Abi as *const <VoiceCommandResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportProgressAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportProgressAsync(&*(&response as *const <VoiceCommandResponse as ::windows::core::Abi>::Abi as *const <VoiceCommandResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportSuccessAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportSuccessAsync(&*(&response as *const <VoiceCommandResponse as ::windows::core::Abi>::Abi as *const <VoiceCommandResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailureAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReportFailureAsync(&*(&response as *const <VoiceCommandResponse as ::windows::core::Abi>::Abi as *const <VoiceCommandResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAppLaunchAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestAppLaunchAsync(&*(&response as *const <VoiceCommandResponse as ::windows::core::Abi>::Abi as *const <VoiceCommandResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VoiceCommandCompleted<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VoiceCommandCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VoiceCommandServiceConnection, VoiceCommandCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VoiceCommandServiceConnection, VoiceCommandCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVoiceCommandCompleted<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveVoiceCommandCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IVoiceCommandServiceConnection>,
            base.5,
            GetVoiceCommandAsync::<Impl, OFFSET>,
            RequestConfirmationAsync::<Impl, OFFSET>,
            RequestDisambiguationAsync::<Impl, OFFSET>,
            ReportProgressAsync::<Impl, OFFSET>,
            ReportSuccessAsync::<Impl, OFFSET>,
            ReportFailureAsync::<Impl, OFFSET>,
            RequestAppLaunchAsync::<Impl, OFFSET>,
            Language::<Impl, OFFSET>,
            VoiceCommandCompleted::<Impl, OFFSET>,
            RemoveVoiceCommandCompleted::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandServiceConnectionStaticsImpl: Sized {
    fn FromAppServiceTriggerDetails(&self, triggerdetails: &::core::option::Option<super::AppService::AppServiceTriggerDetails>) -> ::windows::core::Result<VoiceCommandServiceConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandServiceConnectionStatics {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandServiceConnectionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandServiceConnectionStaticsVtbl {
    pub const fn new<Impl: IVoiceCommandServiceConnectionStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandServiceConnectionStaticsVtbl {
        unsafe extern "system" fn FromAppServiceTriggerDetails<Impl: IVoiceCommandServiceConnectionStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, triggerdetails: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromAppServiceTriggerDetails(&*(&triggerdetails as *const <super::AppService::AppServiceTriggerDetails as ::windows::core::Abi>::Abi as *const <super::AppService::AppServiceTriggerDetails as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommandServiceConnectionStatics>, base.5, FromAppServiceTriggerDetails::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandUserMessageImpl: Sized {
    fn DisplayMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SpokenMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSpokenMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandUserMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandUserMessageVtbl {
    pub const fn new<Impl: IVoiceCommandUserMessageImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVoiceCommandUserMessageVtbl {
        unsafe extern "system" fn DisplayMessage<Impl: IVoiceCommandUserMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayMessage<Impl: IVoiceCommandUserMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisplayMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SpokenMessage<Impl: IVoiceCommandUserMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpokenMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpokenMessage<Impl: IVoiceCommandUserMessageImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSpokenMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVoiceCommandUserMessage>, base.5, DisplayMessage::<Impl, OFFSET>, SetDisplayMessage::<Impl, OFFSET>, SpokenMessage::<Impl, OFFSET>, SetSpokenMessage::<Impl, OFFSET>)
    }
}
