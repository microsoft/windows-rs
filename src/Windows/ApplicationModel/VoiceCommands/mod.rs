#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommand(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommand {
    type Vtable = IVoiceCommand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x936f5273_ec82_42a6_a55c_d2d79ec6f920);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommand_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Media_SpeechRecognition")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandCompletedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommandCompletedEventArgs {
    type Vtable = IVoiceCommandCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc85e675d_fe42_432c_9907_09df9fcf64e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VoiceCommandCompletionReason) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandConfirmationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommandConfirmationResult {
    type Vtable = IVoiceCommandConfirmationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa022593e_8221_4526_b083_840972262247);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandConfirmationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandContentTile(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommandContentTile {
    type Vtable = IVoiceCommandContentTile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3eefe9f0_b8c7_4c76_a0de_1607895ee327);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandContentTile_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut VoiceCommandContentTileType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: VoiceCommandContentTileType) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandDefinition(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommandDefinition {
    type Vtable = IVoiceCommandDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7972aad0_0974_4979_984b_cb8959cd61ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, phraselistname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phraselist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandDefinitionManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommandDefinitionManagerStatics {
    type Vtable = IVoiceCommandDefinitionManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fe7a69e_067e_4f16_a18c_5b17e9499940);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandDefinitionManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandDisambiguationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommandDisambiguationResult {
    type Vtable = IVoiceCommandDisambiguationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc68cfe_c9ac_45df_a8ea_feea08ef9c5e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandDisambiguationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandResponse(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommandResponse {
    type Vtable = IVoiceCommandResponse_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0284b30e_8a3b_4cc4_a6a1_cad5be2716b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandResponse_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandResponseStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommandResponseStatics {
    type Vtable = IVoiceCommandResponseStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2932f813_0d3b_49f2_96dd_625019bd3b5d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandResponseStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, usermessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: ::windows::core::RawPtr, contenttiles: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: ::windows::core::RawPtr, repeatmessage: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: ::windows::core::RawPtr, repeatmessage: ::windows::core::RawPtr, contenttiles: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandServiceConnection(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommandServiceConnection {
    type Vtable = IVoiceCommandServiceConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd894bb9f_21da_44a4_98a2_fb131920a9cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandServiceConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, response: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandServiceConnectionStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommandServiceConnectionStatics {
    type Vtable = IVoiceCommandServiceConnectionStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x370ebffb_2d34_42df_8770_074d0f334697);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandServiceConnectionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_AppService")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, triggerdetails: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IVoiceCommandUserMessage(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IVoiceCommandUserMessage {
    type Vtable = IVoiceCommandUserMessage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x674eb3c0_44f6_4f07_b979_4c723fc08597);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandUserMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoiceCommand(pub ::windows::core::IInspectable);
impl VoiceCommand {
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn CommandName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        }
    }
    #[cfg(feature = "Media_SpeechRecognition")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Media_SpeechRecognition`*"]
    pub fn SpeechRecognitionResult(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommand;{936f5273-ec82-42a6-a55c-d2d79ec6f920})");
}
unsafe impl ::windows::core::Interface for VoiceCommand {
    type Vtable = IVoiceCommand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x936f5273_ec82_42a6_a55c_d2d79ec6f920);
}
impl ::windows::core::RuntimeName for VoiceCommand {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommand";
}
impl ::core::convert::From<VoiceCommand> for ::windows::core::IUnknown {
    fn from(value: VoiceCommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoiceCommand> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoiceCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoiceCommand> for ::windows::core::IInspectable {
    fn from(value: VoiceCommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoiceCommand> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoiceCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VoiceCommand {}
unsafe impl ::core::marker::Sync for VoiceCommand {}
#[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoiceCommandCompletedEventArgs(pub ::windows::core::IInspectable);
impl VoiceCommandCompletedEventArgs {
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn Reason(&self) -> ::windows::core::Result<VoiceCommandCompletionReason> {
        let this = self;
        unsafe {
            let mut result__: VoiceCommandCompletionReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceCommandCompletionReason>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandCompletedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletedEventArgs;{c85e675d-fe42-432c-9907-09df9fcf64e8})");
}
unsafe impl ::windows::core::Interface for VoiceCommandCompletedEventArgs {
    type Vtable = IVoiceCommandCompletedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc85e675d_fe42_432c_9907_09df9fcf64e8);
}
impl ::windows::core::RuntimeName for VoiceCommandCompletedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletedEventArgs";
}
impl ::core::convert::From<VoiceCommandCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandCompletedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoiceCommandCompletedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandCompletedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoiceCommandCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoiceCommandCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoiceCommandCompletedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoiceCommandCompletedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VoiceCommandCompletedEventArgs {}
unsafe impl ::core::marker::Sync for VoiceCommandCompletedEventArgs {}
#[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VoiceCommandCompletionReason(pub i32);
impl VoiceCommandCompletionReason {
    pub const Unknown: VoiceCommandCompletionReason = VoiceCommandCompletionReason(0i32);
    pub const CommunicationFailed: VoiceCommandCompletionReason = VoiceCommandCompletionReason(1i32);
    pub const ResourceLimitsExceeded: VoiceCommandCompletionReason = VoiceCommandCompletionReason(2i32);
    pub const Canceled: VoiceCommandCompletionReason = VoiceCommandCompletionReason(3i32);
    pub const TimeoutExceeded: VoiceCommandCompletionReason = VoiceCommandCompletionReason(4i32);
    pub const AppLaunched: VoiceCommandCompletionReason = VoiceCommandCompletionReason(5i32);
    pub const Completed: VoiceCommandCompletionReason = VoiceCommandCompletionReason(6i32);
}
impl ::core::convert::From<i32> for VoiceCommandCompletionReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VoiceCommandCompletionReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandCompletionReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletionReason;i4)");
}
impl ::windows::core::DefaultType for VoiceCommandCompletionReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoiceCommandConfirmationResult(pub ::windows::core::IInspectable);
impl VoiceCommandConfirmationResult {
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn Confirmed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandConfirmationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandConfirmationResult;{a022593e-8221-4526-b083-840972262247})");
}
unsafe impl ::windows::core::Interface for VoiceCommandConfirmationResult {
    type Vtable = IVoiceCommandConfirmationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa022593e_8221_4526_b083_840972262247);
}
impl ::windows::core::RuntimeName for VoiceCommandConfirmationResult {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandConfirmationResult";
}
impl ::core::convert::From<VoiceCommandConfirmationResult> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandConfirmationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoiceCommandConfirmationResult> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandConfirmationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandConfirmationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoiceCommandConfirmationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoiceCommandConfirmationResult> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandConfirmationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoiceCommandConfirmationResult> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandConfirmationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandConfirmationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoiceCommandConfirmationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VoiceCommandConfirmationResult {}
unsafe impl ::core::marker::Sync for VoiceCommandConfirmationResult {}
#[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoiceCommandContentTile(pub ::windows::core::IInspectable);
impl VoiceCommandContentTile {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoiceCommandContentTile, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn TextLine1(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SetTextLine1<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn TextLine2(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SetTextLine2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn TextLine3(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SetTextLine3<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Storage`*"]
    pub fn Image(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::IStorageFile>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Storage`*"]
    pub fn SetImage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::IStorageFile>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn AppContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SetAppContext<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn AppLaunchArgument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SetAppLaunchArgument<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn ContentTileType(&self) -> ::windows::core::Result<VoiceCommandContentTileType> {
        let this = self;
        unsafe {
            let mut result__: VoiceCommandContentTileType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceCommandContentTileType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SetContentTileType(&self, value: VoiceCommandContentTileType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandContentTile {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTile;{3eefe9f0-b8c7-4c76-a0de-1607895ee327})");
}
unsafe impl ::windows::core::Interface for VoiceCommandContentTile {
    type Vtable = IVoiceCommandContentTile_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3eefe9f0_b8c7_4c76_a0de_1607895ee327);
}
impl ::windows::core::RuntimeName for VoiceCommandContentTile {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTile";
}
impl ::core::convert::From<VoiceCommandContentTile> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandContentTile) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoiceCommandContentTile> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandContentTile) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandContentTile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoiceCommandContentTile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoiceCommandContentTile> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandContentTile) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoiceCommandContentTile> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandContentTile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandContentTile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoiceCommandContentTile {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VoiceCommandContentTile {}
unsafe impl ::core::marker::Sync for VoiceCommandContentTile {}
#[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct VoiceCommandContentTileType(pub i32);
impl VoiceCommandContentTileType {
    pub const TitleOnly: VoiceCommandContentTileType = VoiceCommandContentTileType(0i32);
    pub const TitleWithText: VoiceCommandContentTileType = VoiceCommandContentTileType(1i32);
    pub const TitleWith68x68Icon: VoiceCommandContentTileType = VoiceCommandContentTileType(2i32);
    pub const TitleWith68x68IconAndText: VoiceCommandContentTileType = VoiceCommandContentTileType(3i32);
    pub const TitleWith68x92Icon: VoiceCommandContentTileType = VoiceCommandContentTileType(4i32);
    pub const TitleWith68x92IconAndText: VoiceCommandContentTileType = VoiceCommandContentTileType(5i32);
    pub const TitleWith280x140Icon: VoiceCommandContentTileType = VoiceCommandContentTileType(6i32);
    pub const TitleWith280x140IconAndText: VoiceCommandContentTileType = VoiceCommandContentTileType(7i32);
}
impl ::core::convert::From<i32> for VoiceCommandContentTileType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for VoiceCommandContentTileType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandContentTileType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTileType;i4)");
}
impl ::windows::core::DefaultType for VoiceCommandContentTileType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoiceCommandDefinition(pub ::windows::core::IInspectable);
impl VoiceCommandDefinition {
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation`, `Foundation_Collections`*"]
    pub fn SetPhraseListAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, phraselistname: Param0, phraselist: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), phraselistname.into_param().abi(), phraselist.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition;{7972aad0-0974-4979-984b-cb8959cd61ae})");
}
unsafe impl ::windows::core::Interface for VoiceCommandDefinition {
    type Vtable = IVoiceCommandDefinition_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7972aad0_0974_4979_984b_cb8959cd61ae);
}
impl ::windows::core::RuntimeName for VoiceCommandDefinition {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition";
}
impl ::core::convert::From<VoiceCommandDefinition> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandDefinition) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoiceCommandDefinition> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandDefinition) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoiceCommandDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoiceCommandDefinition> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandDefinition) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoiceCommandDefinition> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoiceCommandDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VoiceCommandDefinition {}
unsafe impl ::core::marker::Sync for VoiceCommandDefinition {}
#[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
pub struct VoiceCommandDefinitionManager {}
impl VoiceCommandDefinitionManager {
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation`, `Storage`*"]
    pub fn InstallCommandDefinitionsFromStorageFileAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(file: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IVoiceCommandDefinitionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation_Collections`*"]
    pub fn InstalledCommandDefinitions() -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandDefinition>> {
        Self::IVoiceCommandDefinitionManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, VoiceCommandDefinition>>(result__)
        })
    }
    pub fn IVoiceCommandDefinitionManagerStatics<R, F: FnOnce(&IVoiceCommandDefinitionManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoiceCommandDefinitionManager, IVoiceCommandDefinitionManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for VoiceCommandDefinitionManager {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager";
}
#[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoiceCommandDisambiguationResult(pub ::windows::core::IInspectable);
impl VoiceCommandDisambiguationResult {
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SelectedItem(&self) -> ::windows::core::Result<VoiceCommandContentTile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceCommandContentTile>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandDisambiguationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandDisambiguationResult;{ecc68cfe-c9ac-45df-a8ea-feea08ef9c5e})");
}
unsafe impl ::windows::core::Interface for VoiceCommandDisambiguationResult {
    type Vtable = IVoiceCommandDisambiguationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc68cfe_c9ac_45df_a8ea_feea08ef9c5e);
}
impl ::windows::core::RuntimeName for VoiceCommandDisambiguationResult {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDisambiguationResult";
}
impl ::core::convert::From<VoiceCommandDisambiguationResult> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandDisambiguationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoiceCommandDisambiguationResult> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandDisambiguationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandDisambiguationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoiceCommandDisambiguationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoiceCommandDisambiguationResult> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandDisambiguationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoiceCommandDisambiguationResult> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandDisambiguationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandDisambiguationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoiceCommandDisambiguationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VoiceCommandDisambiguationResult {}
unsafe impl ::core::marker::Sync for VoiceCommandDisambiguationResult {}
#[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoiceCommandResponse(pub ::windows::core::IInspectable);
impl VoiceCommandResponse {
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn Message(&self) -> ::windows::core::Result<VoiceCommandUserMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceCommandUserMessage>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SetMessage<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn RepeatMessage(&self) -> ::windows::core::Result<VoiceCommandUserMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<VoiceCommandUserMessage>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SetRepeatMessage<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn AppLaunchArgument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SetAppLaunchArgument<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation_Collections`*"]
    pub fn VoiceCommandContentTiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VoiceCommandContentTile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<VoiceCommandContentTile>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn MaxSupportedVoiceCommandContentTiles() -> ::windows::core::Result<u32> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn CreateResponse<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>>(usermessage: Param0) -> ::windows::core::Result<VoiceCommandResponse> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), usermessage.into_param().abi(), &mut result__).from_abi::<VoiceCommandResponse>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation_Collections`*"]
    pub fn CreateResponseWithTiles<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>>(message: Param0, contenttiles: Param1) -> ::windows::core::Result<VoiceCommandResponse> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), message.into_param().abi(), contenttiles.into_param().abi(), &mut result__).from_abi::<VoiceCommandResponse>(result__)
        })
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn CreateResponseForPrompt<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>, Param1: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>>(message: Param0, repeatmessage: Param1) -> ::windows::core::Result<VoiceCommandResponse> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), message.into_param().abi(), repeatmessage.into_param().abi(), &mut result__).from_abi::<VoiceCommandResponse>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation_Collections`*"]
    pub fn CreateResponseForPromptWithTiles<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>, Param1: ::windows::core::IntoParam<'a, VoiceCommandUserMessage>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>>>(message: Param0, repeatmessage: Param1, contenttiles: Param2) -> ::windows::core::Result<VoiceCommandResponse> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), message.into_param().abi(), repeatmessage.into_param().abi(), contenttiles.into_param().abi(), &mut result__).from_abi::<VoiceCommandResponse>(result__)
        })
    }
    pub fn IVoiceCommandResponseStatics<R, F: FnOnce(&IVoiceCommandResponseStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoiceCommandResponse, IVoiceCommandResponseStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandResponse;{0284b30e-8a3b-4cc4-a6a1-cad5be2716b5})");
}
unsafe impl ::windows::core::Interface for VoiceCommandResponse {
    type Vtable = IVoiceCommandResponse_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0284b30e_8a3b_4cc4_a6a1_cad5be2716b5);
}
impl ::windows::core::RuntimeName for VoiceCommandResponse {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandResponse";
}
impl ::core::convert::From<VoiceCommandResponse> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandResponse) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoiceCommandResponse> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandResponse) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoiceCommandResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoiceCommandResponse> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandResponse) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoiceCommandResponse> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandResponse) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoiceCommandResponse {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VoiceCommandResponse {}
unsafe impl ::core::marker::Sync for VoiceCommandResponse {}
#[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoiceCommandServiceConnection(pub ::windows::core::IInspectable);
impl VoiceCommandServiceConnection {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation`*"]
    pub fn GetVoiceCommandAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoiceCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VoiceCommand>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation`*"]
    pub fn RequestConfirmationAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandResponse>>(&self, response: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoiceCommandConfirmationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VoiceCommandConfirmationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation`*"]
    pub fn RequestDisambiguationAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandResponse>>(&self, response: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VoiceCommandDisambiguationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<VoiceCommandDisambiguationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation`*"]
    pub fn ReportProgressAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandResponse>>(&self, response: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation`*"]
    pub fn ReportSuccessAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandResponse>>(&self, response: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation`*"]
    pub fn ReportFailureAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandResponse>>(&self, response: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation`*"]
    pub fn RequestAppLaunchAsync<'a, Param0: ::windows::core::IntoParam<'a, VoiceCommandResponse>>(&self, response: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), response.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Globalization")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Globalization`*"]
    pub fn Language(&self) -> ::windows::core::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Globalization::Language>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation`*"]
    pub fn VoiceCommandCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<VoiceCommandServiceConnection, VoiceCommandCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `Foundation`*"]
    pub fn RemoveVoiceCommandCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "ApplicationModel_AppService")]
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`, `ApplicationModel_AppService`*"]
    pub fn FromAppServiceTriggerDetails<'a, Param0: ::windows::core::IntoParam<'a, super::AppService::AppServiceTriggerDetails>>(triggerdetails: Param0) -> ::windows::core::Result<VoiceCommandServiceConnection> {
        Self::IVoiceCommandServiceConnectionStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), triggerdetails.into_param().abi(), &mut result__).from_abi::<VoiceCommandServiceConnection>(result__)
        })
    }
    pub fn IVoiceCommandServiceConnectionStatics<R, F: FnOnce(&IVoiceCommandServiceConnectionStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoiceCommandServiceConnection, IVoiceCommandServiceConnectionStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandServiceConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandServiceConnection;{d894bb9f-21da-44a4-98a2-fb131920a9cc})");
}
unsafe impl ::windows::core::Interface for VoiceCommandServiceConnection {
    type Vtable = IVoiceCommandServiceConnection_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd894bb9f_21da_44a4_98a2_fb131920a9cc);
}
impl ::windows::core::RuntimeName for VoiceCommandServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandServiceConnection";
}
impl ::core::convert::From<VoiceCommandServiceConnection> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandServiceConnection) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoiceCommandServiceConnection> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandServiceConnection) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoiceCommandServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoiceCommandServiceConnection> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandServiceConnection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoiceCommandServiceConnection> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandServiceConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoiceCommandServiceConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VoiceCommandServiceConnection {}
unsafe impl ::core::marker::Sync for VoiceCommandServiceConnection {}
#[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct VoiceCommandUserMessage(pub ::windows::core::IInspectable);
impl VoiceCommandUserMessage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<VoiceCommandUserMessage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn DisplayMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SetDisplayMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SpokenMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_VoiceCommands`*"]
    pub fn SetSpokenMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for VoiceCommandUserMessage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandUserMessage;{674eb3c0-44f6-4f07-b979-4c723fc08597})");
}
unsafe impl ::windows::core::Interface for VoiceCommandUserMessage {
    type Vtable = IVoiceCommandUserMessage_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x674eb3c0_44f6_4f07_b979_4c723fc08597);
}
impl ::windows::core::RuntimeName for VoiceCommandUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandUserMessage";
}
impl ::core::convert::From<VoiceCommandUserMessage> for ::windows::core::IUnknown {
    fn from(value: VoiceCommandUserMessage) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&VoiceCommandUserMessage> for ::windows::core::IUnknown {
    fn from(value: &VoiceCommandUserMessage) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for VoiceCommandUserMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a VoiceCommandUserMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<VoiceCommandUserMessage> for ::windows::core::IInspectable {
    fn from(value: VoiceCommandUserMessage) -> Self {
        value.0
    }
}
impl ::core::convert::From<&VoiceCommandUserMessage> for ::windows::core::IInspectable {
    fn from(value: &VoiceCommandUserMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for VoiceCommandUserMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a VoiceCommandUserMessage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for VoiceCommandUserMessage {}
unsafe impl ::core::marker::Sync for VoiceCommandUserMessage {}
