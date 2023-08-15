#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommand(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommand {
    type Vtable = IVoiceCommand_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoiceCommand {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x936f5273_ec82_42a6_a55c_d2d79ec6f920);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommand_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CommandName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Media_SpeechRecognition")]
    pub SpeechRecognitionResult: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))]
    SpeechRecognitionResult: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandCompletedEventArgs {
    type Vtable = IVoiceCommandCompletedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandCompletedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc85e675d_fe42_432c_9907_09df9fcf64e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoiceCommandCompletionReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandConfirmationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandConfirmationResult {
    type Vtable = IVoiceCommandConfirmationResult_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandConfirmationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandConfirmationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa022593e_8221_4526_b083_840972262247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandConfirmationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Confirmed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandContentTile(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandContentTile {
    type Vtable = IVoiceCommandContentTile_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandContentTile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandContentTile {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3eefe9f0_b8c7_4c76_a0de_1607895ee327);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandContentTile_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TextLine1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTextLine1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TextLine2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTextLine2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TextLine3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetTextLine3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub Image: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    Image: usize,
    #[cfg(feature = "Storage")]
    pub SetImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetImage: usize,
    pub AppContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAppContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AppLaunchArgument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAppLaunchArgument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContentTileType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VoiceCommandContentTileType) -> ::windows_core::HRESULT,
    pub SetContentTileType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: VoiceCommandContentTileType) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandDefinition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandDefinition {
    type Vtable = IVoiceCommandDefinition_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandDefinition {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7972aad0_0974_4979_984b_cb8959cd61ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandDefinition_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPhraseListAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phraselistname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, phraselist: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPhraseListAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandDefinitionManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandDefinitionManagerStatics {
    type Vtable = IVoiceCommandDefinitionManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandDefinitionManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandDefinitionManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fe7a69e_067e_4f16_a18c_5b17e9499940);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandDefinitionManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub InstallCommandDefinitionsFromStorageFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    InstallCommandDefinitionsFromStorageFileAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub InstalledCommandDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    InstalledCommandDefinitions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandDisambiguationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandDisambiguationResult {
    type Vtable = IVoiceCommandDisambiguationResult_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandDisambiguationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandDisambiguationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecc68cfe_c9ac_45df_a8ea_feea08ef9c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandDisambiguationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandResponse(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandResponse {
    type Vtable = IVoiceCommandResponse_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandResponse {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0284b30e_8a3b_4cc4_a6a1_cad5be2716b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandResponse_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RepeatMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetRepeatMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AppLaunchArgument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetAppLaunchArgument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub VoiceCommandContentTiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VoiceCommandContentTiles: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandResponseStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandResponseStatics {
    type Vtable = IVoiceCommandResponseStatics_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandResponseStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandResponseStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2932f813_0d3b_49f2_96dd_625019bd3b5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandResponseStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MaxSupportedVoiceCommandContentTiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub CreateResponse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usermessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateResponseWithTiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, contenttiles: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateResponseWithTiles: usize,
    pub CreateResponseForPrompt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, repeatmessage: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateResponseForPromptWithTiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: *mut ::core::ffi::c_void, repeatmessage: *mut ::core::ffi::c_void, contenttiles: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateResponseForPromptWithTiles: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandServiceConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandServiceConnection {
    type Vtable = IVoiceCommandServiceConnection_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandServiceConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandServiceConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd894bb9f_21da_44a4_98a2_fb131920a9cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandServiceConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetVoiceCommandAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetVoiceCommandAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestConfirmationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestConfirmationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestDisambiguationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestDisambiguationAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportProgressAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportProgressAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportSuccessAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportSuccessAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailureAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailureAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RequestAppLaunchAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, response: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestAppLaunchAsync: usize,
    #[cfg(feature = "Globalization")]
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Globalization"))]
    Language: usize,
    #[cfg(feature = "Foundation")]
    pub VoiceCommandCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VoiceCommandCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVoiceCommandCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVoiceCommandCompleted: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandServiceConnectionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandServiceConnectionStatics {
    type Vtable = IVoiceCommandServiceConnectionStatics_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandServiceConnectionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandServiceConnectionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x370ebffb_2d34_42df_8770_074d0f334697);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandServiceConnectionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub FromAppServiceTriggerDetails: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, triggerdetails: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    FromAppServiceTriggerDetails: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVoiceCommandUserMessage(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVoiceCommandUserMessage {
    type Vtable = IVoiceCommandUserMessage_Vtbl;
}
impl ::core::clone::Clone for IVoiceCommandUserMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVoiceCommandUserMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x674eb3c0_44f6_4f07_b979_4c723fc08597);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandUserMessage_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DisplayMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDisplayMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SpokenMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetSpokenMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
pub struct VoiceCommand(::windows_core::IUnknown);
impl VoiceCommand {
    pub fn CommandName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CommandName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_SpeechRecognition\"`*"]
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn SpeechRecognitionResult(&self) -> ::windows_core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpeechRecognitionResult)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VoiceCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommand {}
impl ::core::fmt::Debug for VoiceCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommand").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoiceCommand {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommand;{936f5273-ec82-42a6-a55c-d2d79ec6f920})");
}
impl ::core::clone::Clone for VoiceCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VoiceCommand {
    type Vtable = IVoiceCommand_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoiceCommand {
    const IID: ::windows_core::GUID = <IVoiceCommand as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoiceCommand {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommand";
}
::windows_core::imp::interface_hierarchy!(VoiceCommand, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VoiceCommand {}
unsafe impl ::core::marker::Sync for VoiceCommand {}
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
pub struct VoiceCommandCompletedEventArgs(::windows_core::IUnknown);
impl VoiceCommandCompletedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<VoiceCommandCompletionReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VoiceCommandCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandCompletedEventArgs {}
impl ::core::fmt::Debug for VoiceCommandCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoiceCommandCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletedEventArgs;{c85e675d-fe42-432c-9907-09df9fcf64e8})");
}
impl ::core::clone::Clone for VoiceCommandCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VoiceCommandCompletedEventArgs {
    type Vtable = IVoiceCommandCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoiceCommandCompletedEventArgs {
    const IID: ::windows_core::GUID = <IVoiceCommandCompletedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoiceCommandCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletedEventArgs";
}
::windows_core::imp::interface_hierarchy!(VoiceCommandCompletedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VoiceCommandCompletedEventArgs {}
unsafe impl ::core::marker::Sync for VoiceCommandCompletedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
pub struct VoiceCommandConfirmationResult(::windows_core::IUnknown);
impl VoiceCommandConfirmationResult {
    pub fn Confirmed(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Confirmed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VoiceCommandConfirmationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandConfirmationResult {}
impl ::core::fmt::Debug for VoiceCommandConfirmationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandConfirmationResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoiceCommandConfirmationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandConfirmationResult;{a022593e-8221-4526-b083-840972262247})");
}
impl ::core::clone::Clone for VoiceCommandConfirmationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VoiceCommandConfirmationResult {
    type Vtable = IVoiceCommandConfirmationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoiceCommandConfirmationResult {
    const IID: ::windows_core::GUID = <IVoiceCommandConfirmationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoiceCommandConfirmationResult {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandConfirmationResult";
}
::windows_core::imp::interface_hierarchy!(VoiceCommandConfirmationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VoiceCommandConfirmationResult {}
unsafe impl ::core::marker::Sync for VoiceCommandConfirmationResult {}
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
pub struct VoiceCommandContentTile(::windows_core::IUnknown);
impl VoiceCommandContentTile {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VoiceCommandContentTile, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TextLine1(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextLine1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTextLine1(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTextLine1)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TextLine2(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextLine2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTextLine2(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTextLine2)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TextLine3(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextLine3)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTextLine3(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTextLine3)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn Image(&self) -> ::windows_core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Image)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SetImage<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetImage)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn AppContext(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppContext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAppContext<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppContext)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AppLaunchArgument(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppLaunchArgument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAppLaunchArgument(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppLaunchArgument)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn ContentTileType(&self) -> ::windows_core::Result<VoiceCommandContentTileType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentTileType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContentTileType(&self, value: VoiceCommandContentTileType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetContentTileType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for VoiceCommandContentTile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandContentTile {}
impl ::core::fmt::Debug for VoiceCommandContentTile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandContentTile").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoiceCommandContentTile {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTile;{3eefe9f0-b8c7-4c76-a0de-1607895ee327})");
}
impl ::core::clone::Clone for VoiceCommandContentTile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VoiceCommandContentTile {
    type Vtable = IVoiceCommandContentTile_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoiceCommandContentTile {
    const IID: ::windows_core::GUID = <IVoiceCommandContentTile as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoiceCommandContentTile {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTile";
}
::windows_core::imp::interface_hierarchy!(VoiceCommandContentTile, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VoiceCommandContentTile {}
unsafe impl ::core::marker::Sync for VoiceCommandContentTile {}
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
pub struct VoiceCommandDefinition(::windows_core::IUnknown);
impl VoiceCommandDefinition {
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPhraseListAsync<P0>(&self, phraselistname: &::windows_core::HSTRING, phraselist: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetPhraseListAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(phraselistname), phraselist.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VoiceCommandDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandDefinition {}
impl ::core::fmt::Debug for VoiceCommandDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandDefinition").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoiceCommandDefinition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition;{7972aad0-0974-4979-984b-cb8959cd61ae})");
}
impl ::core::clone::Clone for VoiceCommandDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VoiceCommandDefinition {
    type Vtable = IVoiceCommandDefinition_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoiceCommandDefinition {
    const IID: ::windows_core::GUID = <IVoiceCommandDefinition as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoiceCommandDefinition {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition";
}
::windows_core::imp::interface_hierarchy!(VoiceCommandDefinition, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VoiceCommandDefinition {}
unsafe impl ::core::marker::Sync for VoiceCommandDefinition {}
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
pub struct VoiceCommandDefinitionManager;
impl VoiceCommandDefinitionManager {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn InstallCommandDefinitionsFromStorageFileAsync<P0>(file: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::super::Storage::StorageFile>,
    {
        Self::IVoiceCommandDefinitionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstallCommandDefinitionsFromStorageFileAsync)(::windows_core::Interface::as_raw(this), file.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InstalledCommandDefinitions() -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, VoiceCommandDefinition>> {
        Self::IVoiceCommandDefinitionManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InstalledCommandDefinitions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVoiceCommandDefinitionManagerStatics<R, F: FnOnce(&IVoiceCommandDefinitionManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VoiceCommandDefinitionManager, IVoiceCommandDefinitionManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for VoiceCommandDefinitionManager {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager";
}
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
pub struct VoiceCommandDisambiguationResult(::windows_core::IUnknown);
impl VoiceCommandDisambiguationResult {
    pub fn SelectedItem(&self) -> ::windows_core::Result<VoiceCommandContentTile> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedItem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VoiceCommandDisambiguationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandDisambiguationResult {}
impl ::core::fmt::Debug for VoiceCommandDisambiguationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandDisambiguationResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoiceCommandDisambiguationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandDisambiguationResult;{ecc68cfe-c9ac-45df-a8ea-feea08ef9c5e})");
}
impl ::core::clone::Clone for VoiceCommandDisambiguationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VoiceCommandDisambiguationResult {
    type Vtable = IVoiceCommandDisambiguationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoiceCommandDisambiguationResult {
    const IID: ::windows_core::GUID = <IVoiceCommandDisambiguationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoiceCommandDisambiguationResult {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDisambiguationResult";
}
::windows_core::imp::interface_hierarchy!(VoiceCommandDisambiguationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VoiceCommandDisambiguationResult {}
unsafe impl ::core::marker::Sync for VoiceCommandDisambiguationResult {}
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
pub struct VoiceCommandResponse(::windows_core::IUnknown);
impl VoiceCommandResponse {
    pub fn Message(&self) -> ::windows_core::Result<VoiceCommandUserMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Message)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMessage<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VoiceCommandUserMessage>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMessage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn RepeatMessage(&self) -> ::windows_core::Result<VoiceCommandUserMessage> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RepeatMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRepeatMessage<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<VoiceCommandUserMessage>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRepeatMessage)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn AppLaunchArgument(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppLaunchArgument)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAppLaunchArgument(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppLaunchArgument)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn VoiceCommandContentTiles(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<VoiceCommandContentTile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VoiceCommandContentTiles)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaxSupportedVoiceCommandContentTiles() -> ::windows_core::Result<u32> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSupportedVoiceCommandContentTiles)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateResponse<P0>(usermessage: P0) -> ::windows_core::Result<VoiceCommandResponse>
    where
        P0: ::windows_core::IntoParam<VoiceCommandUserMessage>,
    {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResponse)(::windows_core::Interface::as_raw(this), usermessage.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateResponseWithTiles<P0, P1>(message: P0, contenttiles: P1) -> ::windows_core::Result<VoiceCommandResponse>
    where
        P0: ::windows_core::IntoParam<VoiceCommandUserMessage>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>,
    {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResponseWithTiles)(::windows_core::Interface::as_raw(this), message.into_param().abi(), contenttiles.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateResponseForPrompt<P0, P1>(message: P0, repeatmessage: P1) -> ::windows_core::Result<VoiceCommandResponse>
    where
        P0: ::windows_core::IntoParam<VoiceCommandUserMessage>,
        P1: ::windows_core::IntoParam<VoiceCommandUserMessage>,
    {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResponseForPrompt)(::windows_core::Interface::as_raw(this), message.into_param().abi(), repeatmessage.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateResponseForPromptWithTiles<P0, P1, P2>(message: P0, repeatmessage: P1, contenttiles: P2) -> ::windows_core::Result<VoiceCommandResponse>
    where
        P0: ::windows_core::IntoParam<VoiceCommandUserMessage>,
        P1: ::windows_core::IntoParam<VoiceCommandUserMessage>,
        P2: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>,
    {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateResponseForPromptWithTiles)(::windows_core::Interface::as_raw(this), message.into_param().abi(), repeatmessage.into_param().abi(), contenttiles.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVoiceCommandResponseStatics<R, F: FnOnce(&IVoiceCommandResponseStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VoiceCommandResponse, IVoiceCommandResponseStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VoiceCommandResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandResponse {}
impl ::core::fmt::Debug for VoiceCommandResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandResponse").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoiceCommandResponse {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandResponse;{0284b30e-8a3b-4cc4-a6a1-cad5be2716b5})");
}
impl ::core::clone::Clone for VoiceCommandResponse {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VoiceCommandResponse {
    type Vtable = IVoiceCommandResponse_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoiceCommandResponse {
    const IID: ::windows_core::GUID = <IVoiceCommandResponse as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoiceCommandResponse {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandResponse";
}
::windows_core::imp::interface_hierarchy!(VoiceCommandResponse, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VoiceCommandResponse {}
unsafe impl ::core::marker::Sync for VoiceCommandResponse {}
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
pub struct VoiceCommandServiceConnection(::windows_core::IUnknown);
impl VoiceCommandServiceConnection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetVoiceCommandAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VoiceCommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetVoiceCommandAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestConfirmationAsync<P0>(&self, response: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VoiceCommandConfirmationResult>>
    where
        P0: ::windows_core::IntoParam<VoiceCommandResponse>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestConfirmationAsync)(::windows_core::Interface::as_raw(this), response.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestDisambiguationAsync<P0>(&self, response: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<VoiceCommandDisambiguationResult>>
    where
        P0: ::windows_core::IntoParam<VoiceCommandResponse>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestDisambiguationAsync)(::windows_core::Interface::as_raw(this), response.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportProgressAsync<P0>(&self, response: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<VoiceCommandResponse>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportProgressAsync)(::windows_core::Interface::as_raw(this), response.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportSuccessAsync<P0>(&self, response: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<VoiceCommandResponse>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportSuccessAsync)(::windows_core::Interface::as_raw(this), response.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailureAsync<P0>(&self, response: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<VoiceCommandResponse>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailureAsync)(::windows_core::Interface::as_raw(this), response.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestAppLaunchAsync<P0>(&self, response: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<VoiceCommandResponse>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RequestAppLaunchAsync)(::windows_core::Interface::as_raw(this), response.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Globalization\"`*"]
    #[cfg(feature = "Globalization")]
    pub fn Language(&self) -> ::windows_core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VoiceCommandCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VoiceCommandServiceConnection, VoiceCommandCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VoiceCommandCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVoiceCommandCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveVoiceCommandCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_AppService\"`*"]
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn FromAppServiceTriggerDetails<P0>(triggerdetails: P0) -> ::windows_core::Result<VoiceCommandServiceConnection>
    where
        P0: ::windows_core::IntoParam<super::AppService::AppServiceTriggerDetails>,
    {
        Self::IVoiceCommandServiceConnectionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FromAppServiceTriggerDetails)(::windows_core::Interface::as_raw(this), triggerdetails.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVoiceCommandServiceConnectionStatics<R, F: FnOnce(&IVoiceCommandServiceConnectionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VoiceCommandServiceConnection, IVoiceCommandServiceConnectionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VoiceCommandServiceConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandServiceConnection {}
impl ::core::fmt::Debug for VoiceCommandServiceConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandServiceConnection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoiceCommandServiceConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandServiceConnection;{d894bb9f-21da-44a4-98a2-fb131920a9cc})");
}
impl ::core::clone::Clone for VoiceCommandServiceConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VoiceCommandServiceConnection {
    type Vtable = IVoiceCommandServiceConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoiceCommandServiceConnection {
    const IID: ::windows_core::GUID = <IVoiceCommandServiceConnection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoiceCommandServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandServiceConnection";
}
::windows_core::imp::interface_hierarchy!(VoiceCommandServiceConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VoiceCommandServiceConnection {}
unsafe impl ::core::marker::Sync for VoiceCommandServiceConnection {}
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
pub struct VoiceCommandUserMessage(::windows_core::IUnknown);
impl VoiceCommandUserMessage {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VoiceCommandUserMessage, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn DisplayMessage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayMessage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayMessage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn SpokenMessage(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SpokenMessage)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSpokenMessage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSpokenMessage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for VoiceCommandUserMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VoiceCommandUserMessage {}
impl ::core::fmt::Debug for VoiceCommandUserMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandUserMessage").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoiceCommandUserMessage {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandUserMessage;{674eb3c0-44f6-4f07-b979-4c723fc08597})");
}
impl ::core::clone::Clone for VoiceCommandUserMessage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VoiceCommandUserMessage {
    type Vtable = IVoiceCommandUserMessage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VoiceCommandUserMessage {
    const IID: ::windows_core::GUID = <IVoiceCommandUserMessage as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VoiceCommandUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandUserMessage";
}
::windows_core::imp::interface_hierarchy!(VoiceCommandUserMessage, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VoiceCommandUserMessage {}
unsafe impl ::core::marker::Sync for VoiceCommandUserMessage {}
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VoiceCommandCompletionReason(pub i32);
impl VoiceCommandCompletionReason {
    pub const Unknown: Self = Self(0i32);
    pub const CommunicationFailed: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Canceled: Self = Self(3i32);
    pub const TimeoutExceeded: Self = Self(4i32);
    pub const AppLaunched: Self = Self(5i32);
    pub const Completed: Self = Self(6i32);
}
impl ::core::marker::Copy for VoiceCommandCompletionReason {}
impl ::core::clone::Clone for VoiceCommandCompletionReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VoiceCommandCompletionReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VoiceCommandCompletionReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VoiceCommandCompletionReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandCompletionReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoiceCommandCompletionReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletionReason;i4)");
}
#[doc = "*Required features: `\"ApplicationModel_VoiceCommands\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VoiceCommandContentTileType(pub i32);
impl VoiceCommandContentTileType {
    pub const TitleOnly: Self = Self(0i32);
    pub const TitleWithText: Self = Self(1i32);
    pub const TitleWith68x68Icon: Self = Self(2i32);
    pub const TitleWith68x68IconAndText: Self = Self(3i32);
    pub const TitleWith68x92Icon: Self = Self(4i32);
    pub const TitleWith68x92IconAndText: Self = Self(5i32);
    pub const TitleWith280x140Icon: Self = Self(6i32);
    pub const TitleWith280x140IconAndText: Self = Self(7i32);
}
impl ::core::marker::Copy for VoiceCommandContentTileType {}
impl ::core::clone::Clone for VoiceCommandContentTileType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VoiceCommandContentTileType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VoiceCommandContentTileType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VoiceCommandContentTileType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VoiceCommandContentTileType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VoiceCommandContentTileType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTileType;i4)");
}
