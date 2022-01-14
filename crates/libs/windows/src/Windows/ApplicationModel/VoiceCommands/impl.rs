#[cfg(all(feature = "Foundation_Collections", feature = "Media_SpeechRecognition", feature = "implement_exclusive"))]
pub trait IVoiceCommand_Impl: Sized {
    fn CommandName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn SpeechRecognitionResult(&mut self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_SpeechRecognition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommand {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommand";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_SpeechRecognition", feature = "implement_exclusive"))]
impl IVoiceCommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommand_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommand_Vtbl {
        unsafe extern "system" fn CommandName<Impl: IVoiceCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IVoiceCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpeechRecognitionResult<Impl: IVoiceCommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpeechRecognitionResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommand, BASE_OFFSET>(),
            CommandName: CommandName::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            SpeechRecognitionResult: SpeechRecognitionResult::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommand as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandCompletedEventArgs_Impl: Sized {
    fn Reason(&mut self) -> ::windows::core::Result<VoiceCommandCompletionReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandCompletedEventArgs_Vtbl {
        unsafe extern "system" fn Reason<Impl: IVoiceCommandCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VoiceCommandCompletionReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommandCompletedEventArgs, BASE_OFFSET>(), Reason: Reason::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandConfirmationResult_Impl: Sized {
    fn Confirmed(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandConfirmationResult {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandConfirmationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandConfirmationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandConfirmationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandConfirmationResult_Vtbl {
        unsafe extern "system" fn Confirmed<Impl: IVoiceCommandConfirmationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Confirmed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommandConfirmationResult, BASE_OFFSET>(),
            Confirmed: Confirmed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandConfirmationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
pub trait IVoiceCommandContentTile_Impl: Sized {
    fn Title(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TextLine1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTextLine1(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TextLine2(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTextLine2(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TextLine3(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTextLine3(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Image(&mut self) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn SetImage(&mut self, value: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<()>;
    fn AppContext(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetAppContext(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AppLaunchArgument(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAppLaunchArgument(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContentTileType(&mut self) -> ::windows::core::Result<VoiceCommandContentTileType>;
    fn SetContentTileType(&mut self, value: VoiceCommandContentTileType) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandContentTile {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandContentTile";
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl IVoiceCommandContentTile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandContentTile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandContentTile_Vtbl {
        unsafe extern "system" fn Title<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextLine1<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextLine1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextLine1<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextLine1(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextLine2<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextLine2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextLine2<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextLine2(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextLine3<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextLine3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTextLine3<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextLine3(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Image<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&value as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppContext<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppContext<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppLaunchArgument<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppLaunchArgument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppLaunchArgument<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppLaunchArgument(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTileType<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VoiceCommandContentTileType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTileType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentTileType<Impl: IVoiceCommandContentTile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VoiceCommandContentTileType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTileType(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommandContentTile, BASE_OFFSET>(),
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            TextLine1: TextLine1::<Impl, IMPL_OFFSET>,
            SetTextLine1: SetTextLine1::<Impl, IMPL_OFFSET>,
            TextLine2: TextLine2::<Impl, IMPL_OFFSET>,
            SetTextLine2: SetTextLine2::<Impl, IMPL_OFFSET>,
            TextLine3: TextLine3::<Impl, IMPL_OFFSET>,
            SetTextLine3: SetTextLine3::<Impl, IMPL_OFFSET>,
            Image: Image::<Impl, IMPL_OFFSET>,
            SetImage: SetImage::<Impl, IMPL_OFFSET>,
            AppContext: AppContext::<Impl, IMPL_OFFSET>,
            SetAppContext: SetAppContext::<Impl, IMPL_OFFSET>,
            AppLaunchArgument: AppLaunchArgument::<Impl, IMPL_OFFSET>,
            SetAppLaunchArgument: SetAppLaunchArgument::<Impl, IMPL_OFFSET>,
            ContentTileType: ContentTileType::<Impl, IMPL_OFFSET>,
            SetContentTileType: SetContentTileType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandContentTile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVoiceCommandDefinition_Impl: Sized {
    fn Language(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPhraseListAsync(&mut self, phraselistname: &::windows::core::HSTRING, phraselist: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandDefinition {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandDefinition";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVoiceCommandDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandDefinition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandDefinition_Vtbl {
        unsafe extern "system" fn Language<Impl: IVoiceCommandDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IVoiceCommandDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPhraseListAsync<Impl: IVoiceCommandDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraselistname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phraselist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPhraseListAsync(&*(&phraselistname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&phraselist as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommandDefinition, BASE_OFFSET>(),
            Language: Language::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetPhraseListAsync: SetPhraseListAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait IVoiceCommandDefinitionManagerStatics_Impl: Sized {
    fn InstallCommandDefinitionsFromStorageFileAsync(&mut self, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn InstalledCommandDefinitions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandDefinition>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandDefinitionManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandDefinitionManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl IVoiceCommandDefinitionManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandDefinitionManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandDefinitionManagerStatics_Vtbl {
        unsafe extern "system" fn InstallCommandDefinitionsFromStorageFileAsync<Impl: IVoiceCommandDefinitionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallCommandDefinitionsFromStorageFileAsync(&*(&file as *const <super::super::Storage::StorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::StorageFile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstalledCommandDefinitions<Impl: IVoiceCommandDefinitionManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstalledCommandDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommandDefinitionManagerStatics, BASE_OFFSET>(),
            InstallCommandDefinitionsFromStorageFileAsync: InstallCommandDefinitionsFromStorageFileAsync::<Impl, IMPL_OFFSET>,
            InstalledCommandDefinitions: InstalledCommandDefinitions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandDefinitionManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandDisambiguationResult_Impl: Sized {
    fn SelectedItem(&mut self) -> ::windows::core::Result<VoiceCommandContentTile>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandDisambiguationResult {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandDisambiguationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandDisambiguationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandDisambiguationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandDisambiguationResult_Vtbl {
        unsafe extern "system" fn SelectedItem<Impl: IVoiceCommandDisambiguationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommandDisambiguationResult, BASE_OFFSET>(),
            SelectedItem: SelectedItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandDisambiguationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVoiceCommandResponse_Impl: Sized {
    fn Message(&mut self) -> ::windows::core::Result<VoiceCommandUserMessage>;
    fn SetMessage(&mut self, value: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<()>;
    fn RepeatMessage(&mut self) -> ::windows::core::Result<VoiceCommandUserMessage>;
    fn SetRepeatMessage(&mut self, value: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<()>;
    fn AppLaunchArgument(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAppLaunchArgument(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn VoiceCommandContentTiles(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VoiceCommandContentTile>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandResponse {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandResponse";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVoiceCommandResponse_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandResponse_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandResponse_Vtbl {
        unsafe extern "system" fn Message<Impl: IVoiceCommandResponse_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessage<Impl: IVoiceCommandResponse_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessage(&*(&value as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RepeatMessage<Impl: IVoiceCommandResponse_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RepeatMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepeatMessage<Impl: IVoiceCommandResponse_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRepeatMessage(&*(&value as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppLaunchArgument<Impl: IVoiceCommandResponse_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppLaunchArgument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppLaunchArgument<Impl: IVoiceCommandResponse_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppLaunchArgument(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VoiceCommandContentTiles<Impl: IVoiceCommandResponse_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VoiceCommandContentTiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommandResponse, BASE_OFFSET>(),
            Message: Message::<Impl, IMPL_OFFSET>,
            SetMessage: SetMessage::<Impl, IMPL_OFFSET>,
            RepeatMessage: RepeatMessage::<Impl, IMPL_OFFSET>,
            SetRepeatMessage: SetRepeatMessage::<Impl, IMPL_OFFSET>,
            AppLaunchArgument: AppLaunchArgument::<Impl, IMPL_OFFSET>,
            SetAppLaunchArgument: SetAppLaunchArgument::<Impl, IMPL_OFFSET>,
            VoiceCommandContentTiles: VoiceCommandContentTiles::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandResponse as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVoiceCommandResponseStatics_Impl: Sized {
    fn MaxSupportedVoiceCommandContentTiles(&mut self) -> ::windows::core::Result<u32>;
    fn CreateResponse(&mut self, usermessage: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<VoiceCommandResponse>;
    fn CreateResponseWithTiles(&mut self, message: &::core::option::Option<VoiceCommandUserMessage>, contenttiles: &::core::option::Option<super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>) -> ::windows::core::Result<VoiceCommandResponse>;
    fn CreateResponseForPrompt(&mut self, message: &::core::option::Option<VoiceCommandUserMessage>, repeatmessage: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<VoiceCommandResponse>;
    fn CreateResponseForPromptWithTiles(&mut self, message: &::core::option::Option<VoiceCommandUserMessage>, repeatmessage: &::core::option::Option<VoiceCommandUserMessage>, contenttiles: &::core::option::Option<super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>) -> ::windows::core::Result<VoiceCommandResponse>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandResponseStatics {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandResponseStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVoiceCommandResponseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandResponseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandResponseStatics_Vtbl {
        unsafe extern "system" fn MaxSupportedVoiceCommandContentTiles<Impl: IVoiceCommandResponseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSupportedVoiceCommandContentTiles() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResponse<Impl: IVoiceCommandResponseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usermessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResponse(&*(&usermessage as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResponseWithTiles<Impl: IVoiceCommandResponseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, contenttiles: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResponseWithTiles(&*(&message as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType), &*(&contenttiles as *const <super::super::Foundation::Collections::IIterable<VoiceCommandContentTile> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<VoiceCommandContentTile> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResponseForPrompt<Impl: IVoiceCommandResponseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, repeatmessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResponseForPrompt(&*(&message as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType), &*(&repeatmessage as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResponseForPromptWithTiles<Impl: IVoiceCommandResponseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, repeatmessage: ::windows::core::RawPtr, contenttiles: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommandResponseStatics, BASE_OFFSET>(),
            MaxSupportedVoiceCommandContentTiles: MaxSupportedVoiceCommandContentTiles::<Impl, IMPL_OFFSET>,
            CreateResponse: CreateResponse::<Impl, IMPL_OFFSET>,
            CreateResponseWithTiles: CreateResponseWithTiles::<Impl, IMPL_OFFSET>,
            CreateResponseForPrompt: CreateResponseForPrompt::<Impl, IMPL_OFFSET>,
            CreateResponseForPromptWithTiles: CreateResponseForPromptWithTiles::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandResponseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
pub trait IVoiceCommandServiceConnection_Impl: Sized {
    fn GetVoiceCommandAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoiceCommand>>;
    fn RequestConfirmationAsync(&mut self, response: &::core::option::Option<VoiceCommandResponse>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoiceCommandConfirmationResult>>;
    fn RequestDisambiguationAsync(&mut self, response: &::core::option::Option<VoiceCommandResponse>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoiceCommandDisambiguationResult>>;
    fn ReportProgressAsync(&mut self, response: &::core::option::Option<VoiceCommandResponse>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ReportSuccessAsync(&mut self, response: &::core::option::Option<VoiceCommandResponse>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ReportFailureAsync(&mut self, response: &::core::option::Option<VoiceCommandResponse>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestAppLaunchAsync(&mut self, response: &::core::option::Option<VoiceCommandResponse>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn Language(&mut self) -> ::windows::core::Result<super::super::Globalization::Language>;
    fn VoiceCommandCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VoiceCommandServiceConnection, VoiceCommandCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVoiceCommandCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandServiceConnection";
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
impl IVoiceCommandServiceConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandServiceConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandServiceConnection_Vtbl {
        unsafe extern "system" fn GetVoiceCommandAsync<Impl: IVoiceCommandServiceConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVoiceCommandAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestConfirmationAsync<Impl: IVoiceCommandServiceConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestConfirmationAsync(&*(&response as *const <VoiceCommandResponse as ::windows::core::Abi>::Abi as *const <VoiceCommandResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestDisambiguationAsync<Impl: IVoiceCommandServiceConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestDisambiguationAsync(&*(&response as *const <VoiceCommandResponse as ::windows::core::Abi>::Abi as *const <VoiceCommandResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportProgressAsync<Impl: IVoiceCommandServiceConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportProgressAsync(&*(&response as *const <VoiceCommandResponse as ::windows::core::Abi>::Abi as *const <VoiceCommandResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportSuccessAsync<Impl: IVoiceCommandServiceConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportSuccessAsync(&*(&response as *const <VoiceCommandResponse as ::windows::core::Abi>::Abi as *const <VoiceCommandResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailureAsync<Impl: IVoiceCommandServiceConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailureAsync(&*(&response as *const <VoiceCommandResponse as ::windows::core::Abi>::Abi as *const <VoiceCommandResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAppLaunchAsync<Impl: IVoiceCommandServiceConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAppLaunchAsync(&*(&response as *const <VoiceCommandResponse as ::windows::core::Abi>::Abi as *const <VoiceCommandResponse as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Language<Impl: IVoiceCommandServiceConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VoiceCommandCompleted<Impl: IVoiceCommandServiceConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VoiceCommandCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VoiceCommandServiceConnection, VoiceCommandCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VoiceCommandServiceConnection, VoiceCommandCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVoiceCommandCompleted<Impl: IVoiceCommandServiceConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVoiceCommandCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommandServiceConnection, BASE_OFFSET>(),
            GetVoiceCommandAsync: GetVoiceCommandAsync::<Impl, IMPL_OFFSET>,
            RequestConfirmationAsync: RequestConfirmationAsync::<Impl, IMPL_OFFSET>,
            RequestDisambiguationAsync: RequestDisambiguationAsync::<Impl, IMPL_OFFSET>,
            ReportProgressAsync: ReportProgressAsync::<Impl, IMPL_OFFSET>,
            ReportSuccessAsync: ReportSuccessAsync::<Impl, IMPL_OFFSET>,
            ReportFailureAsync: ReportFailureAsync::<Impl, IMPL_OFFSET>,
            RequestAppLaunchAsync: RequestAppLaunchAsync::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            VoiceCommandCompleted: VoiceCommandCompleted::<Impl, IMPL_OFFSET>,
            RemoveVoiceCommandCompleted: RemoveVoiceCommandCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandServiceConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
pub trait IVoiceCommandServiceConnectionStatics_Impl: Sized {
    fn FromAppServiceTriggerDetails(&mut self, triggerdetails: &::core::option::Option<super::AppService::AppServiceTriggerDetails>) -> ::windows::core::Result<VoiceCommandServiceConnection>;
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandServiceConnectionStatics {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandServiceConnectionStatics";
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
impl IVoiceCommandServiceConnectionStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandServiceConnectionStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandServiceConnectionStatics_Vtbl {
        unsafe extern "system" fn FromAppServiceTriggerDetails<Impl: IVoiceCommandServiceConnectionStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triggerdetails: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromAppServiceTriggerDetails(&*(&triggerdetails as *const <super::AppService::AppServiceTriggerDetails as ::windows::core::Abi>::Abi as *const <super::AppService::AppServiceTriggerDetails as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommandServiceConnectionStatics, BASE_OFFSET>(),
            FromAppServiceTriggerDetails: FromAppServiceTriggerDetails::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandServiceConnectionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandUserMessage_Impl: Sized {
    fn DisplayMessage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayMessage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SpokenMessage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSpokenMessage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVoiceCommandUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandUserMessage";
}
#[cfg(feature = "implement_exclusive")]
impl IVoiceCommandUserMessage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandUserMessage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandUserMessage_Vtbl {
        unsafe extern "system" fn DisplayMessage<Impl: IVoiceCommandUserMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayMessage<Impl: IVoiceCommandUserMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SpokenMessage<Impl: IVoiceCommandUserMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpokenMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpokenMessage<Impl: IVoiceCommandUserMessage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpokenMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVoiceCommandUserMessage, BASE_OFFSET>(),
            DisplayMessage: DisplayMessage::<Impl, IMPL_OFFSET>,
            SetDisplayMessage: SetDisplayMessage::<Impl, IMPL_OFFSET>,
            SpokenMessage: SpokenMessage::<Impl, IMPL_OFFSET>,
            SetSpokenMessage: SetSpokenMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandUserMessage as ::windows::core::Interface>::IID
    }
}
