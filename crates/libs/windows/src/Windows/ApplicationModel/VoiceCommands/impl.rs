#[cfg(all(feature = "Foundation_Collections", feature = "Media_SpeechRecognition", feature = "implement_exclusive"))]
pub trait IVoiceCommandImpl: Sized {
    fn CommandName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn SpeechRecognitionResult(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_SpeechRecognition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommand {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommand";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Media_SpeechRecognition", feature = "implement_exclusive"))]
impl IVoiceCommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandVtbl {
        unsafe extern "system" fn CommandName<Impl: IVoiceCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Properties<Impl: IVoiceCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SpeechRecognitionResult<Impl: IVoiceCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVoiceCommand>, ::windows::core::GetTrustLevel, CommandName::<Impl, IMPL_OFFSET>, Properties::<Impl, IMPL_OFFSET>, SpeechRecognitionResult::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommand as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandCompletedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IVoiceCommandCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VoiceCommandCompletionReason) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVoiceCommandCompletedEventArgs>, ::windows::core::GetTrustLevel, Reason::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandCompletedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandConfirmationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandConfirmationResultVtbl {
        unsafe extern "system" fn Confirmed<Impl: IVoiceCommandConfirmationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVoiceCommandConfirmationResult>, ::windows::core::GetTrustLevel, Confirmed::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandConfirmationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandContentTile {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandContentTile";
}
#[cfg(all(feature = "Storage", feature = "implement_exclusive"))]
impl IVoiceCommandContentTileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandContentTileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandContentTileVtbl {
        unsafe extern "system" fn Title<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextLine1<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTextLine1<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextLine1(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextLine2<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTextLine2<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextLine2(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TextLine3<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTextLine3<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTextLine3(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Image<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetImage<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImage(&*(&value as *const <super::super::Storage::IStorageFile as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFile as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppContext<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAppContext<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppLaunchArgument<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAppLaunchArgument<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppLaunchArgument(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentTileType<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VoiceCommandContentTileType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentTileType<Impl: IVoiceCommandContentTileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VoiceCommandContentTileType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentTileType(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVoiceCommandContentTile>,
            ::windows::core::GetTrustLevel,
            Title::<Impl, IMPL_OFFSET>,
            SetTitle::<Impl, IMPL_OFFSET>,
            TextLine1::<Impl, IMPL_OFFSET>,
            SetTextLine1::<Impl, IMPL_OFFSET>,
            TextLine2::<Impl, IMPL_OFFSET>,
            SetTextLine2::<Impl, IMPL_OFFSET>,
            TextLine3::<Impl, IMPL_OFFSET>,
            SetTextLine3::<Impl, IMPL_OFFSET>,
            Image::<Impl, IMPL_OFFSET>,
            SetImage::<Impl, IMPL_OFFSET>,
            AppContext::<Impl, IMPL_OFFSET>,
            SetAppContext::<Impl, IMPL_OFFSET>,
            AppLaunchArgument::<Impl, IMPL_OFFSET>,
            SetAppLaunchArgument::<Impl, IMPL_OFFSET>,
            ContentTileType::<Impl, IMPL_OFFSET>,
            SetContentTileType::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandContentTile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVoiceCommandDefinitionImpl: Sized {
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPhraseListAsync(&self, phraselistname: &::windows::core::HSTRING, phraselist: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandDefinition {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandDefinition";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVoiceCommandDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandDefinitionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandDefinitionVtbl {
        unsafe extern "system" fn Language<Impl: IVoiceCommandDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Name<Impl: IVoiceCommandDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPhraseListAsync<Impl: IVoiceCommandDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phraselistname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phraselist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVoiceCommandDefinition>, ::windows::core::GetTrustLevel, Language::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, SetPhraseListAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
pub trait IVoiceCommandDefinitionManagerStaticsImpl: Sized {
    fn InstallCommandDefinitionsFromStorageFileAsync(&self, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn InstalledCommandDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandDefinition>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandDefinitionManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandDefinitionManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage", feature = "implement_exclusive"))]
impl IVoiceCommandDefinitionManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandDefinitionManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandDefinitionManagerStaticsVtbl {
        unsafe extern "system" fn InstallCommandDefinitionsFromStorageFileAsync<Impl: IVoiceCommandDefinitionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InstalledCommandDefinitions<Impl: IVoiceCommandDefinitionManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVoiceCommandDefinitionManagerStatics>, ::windows::core::GetTrustLevel, InstallCommandDefinitionsFromStorageFileAsync::<Impl, IMPL_OFFSET>, InstalledCommandDefinitions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandDefinitionManagerStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandDisambiguationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandDisambiguationResultVtbl {
        unsafe extern "system" fn SelectedItem<Impl: IVoiceCommandDisambiguationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVoiceCommandDisambiguationResult>, ::windows::core::GetTrustLevel, SelectedItem::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandDisambiguationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVoiceCommandResponseImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<VoiceCommandUserMessage>;
    fn SetMessage(&self, value: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<()>;
    fn RepeatMessage(&self) -> ::windows::core::Result<VoiceCommandUserMessage>;
    fn SetRepeatMessage(&self, value: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<()>;
    fn AppLaunchArgument(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAppLaunchArgument(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn VoiceCommandContentTiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VoiceCommandContentTile>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandResponse {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandResponse";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVoiceCommandResponseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandResponseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandResponseVtbl {
        unsafe extern "system" fn Message<Impl: IVoiceCommandResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMessage<Impl: IVoiceCommandResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessage(&*(&value as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RepeatMessage<Impl: IVoiceCommandResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRepeatMessage<Impl: IVoiceCommandResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRepeatMessage(&*(&value as *const <VoiceCommandUserMessage as ::windows::core::Abi>::Abi as *const <VoiceCommandUserMessage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppLaunchArgument<Impl: IVoiceCommandResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAppLaunchArgument<Impl: IVoiceCommandResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppLaunchArgument(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VoiceCommandContentTiles<Impl: IVoiceCommandResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVoiceCommandResponse>,
            ::windows::core::GetTrustLevel,
            Message::<Impl, IMPL_OFFSET>,
            SetMessage::<Impl, IMPL_OFFSET>,
            RepeatMessage::<Impl, IMPL_OFFSET>,
            SetRepeatMessage::<Impl, IMPL_OFFSET>,
            AppLaunchArgument::<Impl, IMPL_OFFSET>,
            SetAppLaunchArgument::<Impl, IMPL_OFFSET>,
            VoiceCommandContentTiles::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandResponse as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVoiceCommandResponseStaticsImpl: Sized {
    fn MaxSupportedVoiceCommandContentTiles(&self) -> ::windows::core::Result<u32>;
    fn CreateResponse(&self, usermessage: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<VoiceCommandResponse>;
    fn CreateResponseWithTiles(&self, message: &::core::option::Option<VoiceCommandUserMessage>, contenttiles: &::core::option::Option<super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>) -> ::windows::core::Result<VoiceCommandResponse>;
    fn CreateResponseForPrompt(&self, message: &::core::option::Option<VoiceCommandUserMessage>, repeatmessage: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<VoiceCommandResponse>;
    fn CreateResponseForPromptWithTiles(&self, message: &::core::option::Option<VoiceCommandUserMessage>, repeatmessage: &::core::option::Option<VoiceCommandUserMessage>, contenttiles: &::core::option::Option<super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>) -> ::windows::core::Result<VoiceCommandResponse>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandResponseStatics {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandResponseStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVoiceCommandResponseStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandResponseStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandResponseStaticsVtbl {
        unsafe extern "system" fn MaxSupportedVoiceCommandContentTiles<Impl: IVoiceCommandResponseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateResponse<Impl: IVoiceCommandResponseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, usermessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateResponseWithTiles<Impl: IVoiceCommandResponseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, contenttiles: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateResponseForPrompt<Impl: IVoiceCommandResponseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, repeatmessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateResponseForPromptWithTiles<Impl: IVoiceCommandResponseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, repeatmessage: ::windows::core::RawPtr, contenttiles: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVoiceCommandResponseStatics>,
            ::windows::core::GetTrustLevel,
            MaxSupportedVoiceCommandContentTiles::<Impl, IMPL_OFFSET>,
            CreateResponse::<Impl, IMPL_OFFSET>,
            CreateResponseWithTiles::<Impl, IMPL_OFFSET>,
            CreateResponseForPrompt::<Impl, IMPL_OFFSET>,
            CreateResponseForPromptWithTiles::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandResponseStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandServiceConnection";
}
#[cfg(all(feature = "Foundation", feature = "Globalization", feature = "implement_exclusive"))]
impl IVoiceCommandServiceConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandServiceConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandServiceConnectionVtbl {
        unsafe extern "system" fn GetVoiceCommandAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestConfirmationAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestDisambiguationAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReportProgressAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReportSuccessAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReportFailureAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAppLaunchAsync<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Language<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VoiceCommandCompleted<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveVoiceCommandCompleted<Impl: IVoiceCommandServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVoiceCommandCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IVoiceCommandServiceConnection>,
            ::windows::core::GetTrustLevel,
            GetVoiceCommandAsync::<Impl, IMPL_OFFSET>,
            RequestConfirmationAsync::<Impl, IMPL_OFFSET>,
            RequestDisambiguationAsync::<Impl, IMPL_OFFSET>,
            ReportProgressAsync::<Impl, IMPL_OFFSET>,
            ReportSuccessAsync::<Impl, IMPL_OFFSET>,
            ReportFailureAsync::<Impl, IMPL_OFFSET>,
            RequestAppLaunchAsync::<Impl, IMPL_OFFSET>,
            Language::<Impl, IMPL_OFFSET>,
            VoiceCommandCompleted::<Impl, IMPL_OFFSET>,
            RemoveVoiceCommandCompleted::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandServiceConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
pub trait IVoiceCommandServiceConnectionStaticsImpl: Sized {
    fn FromAppServiceTriggerDetails(&self, triggerdetails: &::core::option::Option<super::AppService::AppServiceTriggerDetails>) -> ::windows::core::Result<VoiceCommandServiceConnection>;
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVoiceCommandServiceConnectionStatics {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.IVoiceCommandServiceConnectionStatics";
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
impl IVoiceCommandServiceConnectionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandServiceConnectionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandServiceConnectionStaticsVtbl {
        unsafe extern "system" fn FromAppServiceTriggerDetails<Impl: IVoiceCommandServiceConnectionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, triggerdetails: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVoiceCommandServiceConnectionStatics>, ::windows::core::GetTrustLevel, FromAppServiceTriggerDetails::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandServiceConnectionStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVoiceCommandUserMessageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVoiceCommandUserMessageVtbl {
        unsafe extern "system" fn DisplayMessage<Impl: IVoiceCommandUserMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisplayMessage<Impl: IVoiceCommandUserMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SpokenMessage<Impl: IVoiceCommandUserMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSpokenMessage<Impl: IVoiceCommandUserMessageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpokenMessage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IVoiceCommandUserMessage>, ::windows::core::GetTrustLevel, DisplayMessage::<Impl, IMPL_OFFSET>, SetDisplayMessage::<Impl, IMPL_OFFSET>, SpokenMessage::<Impl, IMPL_OFFSET>, SetSpokenMessage::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVoiceCommandUserMessage as ::windows::core::Interface>::IID
    }
}
