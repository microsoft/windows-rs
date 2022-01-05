#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandImpl: Sized {
    fn CommandName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
    fn SpeechRecognitionResult(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandCompletedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<VoiceCommandCompletionReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandConfirmationResultImpl: Sized {
    fn Confirmed(&self) -> ::windows::core::Result<bool>;
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
pub trait IVoiceCommandDefinitionImpl: Sized {
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPhraseListAsync(&self, phraselistname: &::windows::core::HSTRING, phraselist: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandDefinitionManagerStaticsImpl: Sized {
    fn InstallCommandDefinitionsFromStorageFileAsync(&self, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn InstalledCommandDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandDefinition>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandDisambiguationResultImpl: Sized {
    fn SelectedItem(&self) -> ::windows::core::Result<VoiceCommandContentTile>;
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
pub trait IVoiceCommandResponseStaticsImpl: Sized {
    fn MaxSupportedVoiceCommandContentTiles(&self) -> ::windows::core::Result<u32>;
    fn CreateResponse(&self, usermessage: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<VoiceCommandResponse>;
    fn CreateResponseWithTiles(&self, message: &::core::option::Option<VoiceCommandUserMessage>, contenttiles: &::core::option::Option<super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>) -> ::windows::core::Result<VoiceCommandResponse>;
    fn CreateResponseForPrompt(&self, message: &::core::option::Option<VoiceCommandUserMessage>, repeatmessage: &::core::option::Option<VoiceCommandUserMessage>) -> ::windows::core::Result<VoiceCommandResponse>;
    fn CreateResponseForPromptWithTiles(&self, message: &::core::option::Option<VoiceCommandUserMessage>, repeatmessage: &::core::option::Option<VoiceCommandUserMessage>, contenttiles: &::core::option::Option<super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>) -> ::windows::core::Result<VoiceCommandResponse>;
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
pub trait IVoiceCommandServiceConnectionStaticsImpl: Sized {
    fn FromAppServiceTriggerDetails(&self, triggerdetails: &::core::option::Option<super::AppService::AppServiceTriggerDetails>) -> ::windows::core::Result<VoiceCommandServiceConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVoiceCommandUserMessageImpl: Sized {
    fn DisplayMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SpokenMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSpokenMessage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
