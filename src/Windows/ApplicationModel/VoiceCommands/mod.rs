#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVoiceCommand(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommand {
    type Vtable = IVoiceCommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2473546355,
        60546,
        17062,
        [165, 92, 210, 215, 158, 198, 249, 32],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommand_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Media_SpeechRecognition")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_SpeechRecognition"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVoiceCommandCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandCompletedEventArgs {
    type Vtable = IVoiceCommandCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3361630045,
        65090,
        17196,
        [153, 7, 9, 223, 159, 207, 100, 232],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandCompletedEventArgs_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut VoiceCommandCompletionReason,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVoiceCommandConfirmationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandConfirmationResult {
    type Vtable = IVoiceCommandConfirmationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2686605630,
        33313,
        17702,
        [176, 131, 132, 9, 114, 38, 34, 71],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandConfirmationResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVoiceCommandContentTile(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandContentTile {
    type Vtable = IVoiceCommandContentTile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1055910384,
        47303,
        19574,
        [160, 222, 22, 7, 137, 94, 227, 39],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandContentTile_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    #[cfg(feature = "Storage")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut VoiceCommandContentTileType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: VoiceCommandContentTileType,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVoiceCommandDefinition(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandDefinition {
    type Vtable = IVoiceCommandDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2037557968,
        2420,
        18809,
        [152, 75, 203, 137, 89, 205, 97, 174],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandDefinition_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        phraselistname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        phraselist: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVoiceCommandDefinitionManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandDefinitionManagerStatics {
    type Vtable = IVoiceCommandDefinitionManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2414323358,
        1662,
        20246,
        [161, 140, 91, 23, 233, 73, 153, 64],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandDefinitionManagerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        file: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVoiceCommandDisambiguationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandDisambiguationResult {
    type Vtable = IVoiceCommandDisambiguationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3972435198,
        51628,
        17887,
        [168, 234, 254, 234, 8, 239, 156, 94],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandDisambiguationResult_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVoiceCommandResponse(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandResponse {
    type Vtable = IVoiceCommandResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        42251022,
        35387,
        19652,
        [166, 161, 202, 213, 190, 39, 22, 181],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandResponse_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVoiceCommandResponseStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandResponseStatics {
    type Vtable = IVoiceCommandResponseStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        691206163,
        3387,
        18930,
        [150, 221, 98, 80, 25, 189, 59, 93],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandResponseStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        usermessage: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        message: ::windows::runtime::RawPtr,
        contenttiles: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        message: ::windows::runtime::RawPtr,
        repeatmessage: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        message: ::windows::runtime::RawPtr,
        repeatmessage: ::windows::runtime::RawPtr,
        contenttiles: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVoiceCommandServiceConnection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandServiceConnection {
    type Vtable = IVoiceCommandServiceConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3633626015,
        8666,
        17572,
        [152, 162, 251, 19, 25, 32, 169, 204],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandServiceConnection_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        response: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        response: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        response: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        response: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        response: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        response: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Globalization")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVoiceCommandServiceConnectionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandServiceConnectionStatics {
    type Vtable = IVoiceCommandServiceConnectionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        923713531,
        11572,
        17119,
        [135, 112, 7, 77, 15, 51, 70, 151],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandServiceConnectionStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        triggerdetails: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IVoiceCommandUserMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IVoiceCommandUserMessage {
    type Vtable = IVoiceCommandUserMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1733211072,
        17654,
        20231,
        [185, 121, 76, 114, 63, 192, 133, 151],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IVoiceCommandUserMessage_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct VoiceCommand(::windows::runtime::IInspectable);
impl VoiceCommand {
    pub fn CommandName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMapView<
            ::windows::runtime::HSTRING,
            super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMapView<
                ::windows::runtime::HSTRING,
                super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>,
            >>(result__)
        }
    }
    #[cfg(feature = "Media_SpeechRecognition")]
    pub fn SpeechRecognitionResult(
        &self,
    ) -> ::windows::runtime::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommand {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommand;{936f5273-ec82-42a6-a55c-d2d79ec6f920})" ) ;
}
unsafe impl ::windows::runtime::Interface for VoiceCommand {
    type Vtable = IVoiceCommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2473546355,
        60546,
        17062,
        [165, 92, 210, 215, 158, 198, 249, 32],
    );
}
impl ::windows::runtime::RuntimeName for VoiceCommand {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommand";
}
impl ::std::convert::From<VoiceCommand> for ::windows::runtime::IUnknown {
    fn from(value: VoiceCommand) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VoiceCommand> for ::windows::runtime::IUnknown {
    fn from(value: &VoiceCommand) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VoiceCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VoiceCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<VoiceCommand> for ::windows::runtime::IInspectable {
    fn from(value: VoiceCommand) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceCommand> for ::windows::runtime::IInspectable {
    fn from(value: &VoiceCommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for VoiceCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a VoiceCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VoiceCommand {}
unsafe impl ::std::marker::Sync for VoiceCommand {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct VoiceCommandCompletedEventArgs(::windows::runtime::IInspectable);
impl VoiceCommandCompletedEventArgs {
    pub fn Reason(&self) -> ::windows::runtime::Result<VoiceCommandCompletionReason> {
        let this = self;
        unsafe {
            let mut result__: VoiceCommandCompletionReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<VoiceCommandCompletionReason>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommandCompletedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletedEventArgs;{c85e675d-fe42-432c-9907-09df9fcf64e8})" ) ;
}
unsafe impl ::windows::runtime::Interface for VoiceCommandCompletedEventArgs {
    type Vtable = IVoiceCommandCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3361630045,
        65090,
        17196,
        [153, 7, 9, 223, 159, 207, 100, 232],
    );
}
impl ::windows::runtime::RuntimeName for VoiceCommandCompletedEventArgs {
    const NAME: &'static str =
        "Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletedEventArgs";
}
impl ::std::convert::From<VoiceCommandCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: VoiceCommandCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VoiceCommandCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &VoiceCommandCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for VoiceCommandCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &VoiceCommandCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<VoiceCommandCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: VoiceCommandCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceCommandCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &VoiceCommandCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for VoiceCommandCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a VoiceCommandCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VoiceCommandCompletedEventArgs {}
unsafe impl ::std::marker::Sync for VoiceCommandCompletedEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VoiceCommandCompletionReason(pub i32);
impl VoiceCommandCompletionReason {
    pub const Unknown: VoiceCommandCompletionReason = VoiceCommandCompletionReason(0i32);
    pub const CommunicationFailed: VoiceCommandCompletionReason =
        VoiceCommandCompletionReason(1i32);
    pub const ResourceLimitsExceeded: VoiceCommandCompletionReason =
        VoiceCommandCompletionReason(2i32);
    pub const Canceled: VoiceCommandCompletionReason = VoiceCommandCompletionReason(3i32);
    pub const TimeoutExceeded: VoiceCommandCompletionReason = VoiceCommandCompletionReason(4i32);
    pub const AppLaunched: VoiceCommandCompletionReason = VoiceCommandCompletionReason(5i32);
    pub const Completed: VoiceCommandCompletionReason = VoiceCommandCompletionReason(6i32);
}
impl ::std::convert::From<i32> for VoiceCommandCompletionReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VoiceCommandCompletionReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommandCompletionReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.VoiceCommands.VoiceCommandCompletionReason;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct VoiceCommandConfirmationResult(::windows::runtime::IInspectable);
impl VoiceCommandConfirmationResult {
    pub fn Confirmed(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommandConfirmationResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandConfirmationResult;{a022593e-8221-4526-b083-840972262247})" ) ;
}
unsafe impl ::windows::runtime::Interface for VoiceCommandConfirmationResult {
    type Vtable = IVoiceCommandConfirmationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2686605630,
        33313,
        17702,
        [176, 131, 132, 9, 114, 38, 34, 71],
    );
}
impl ::windows::runtime::RuntimeName for VoiceCommandConfirmationResult {
    const NAME: &'static str =
        "Windows.ApplicationModel.VoiceCommands.VoiceCommandConfirmationResult";
}
impl ::std::convert::From<VoiceCommandConfirmationResult> for ::windows::runtime::IUnknown {
    fn from(value: VoiceCommandConfirmationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VoiceCommandConfirmationResult> for ::windows::runtime::IUnknown {
    fn from(value: &VoiceCommandConfirmationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for VoiceCommandConfirmationResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &VoiceCommandConfirmationResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<VoiceCommandConfirmationResult> for ::windows::runtime::IInspectable {
    fn from(value: VoiceCommandConfirmationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceCommandConfirmationResult> for ::windows::runtime::IInspectable {
    fn from(value: &VoiceCommandConfirmationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for VoiceCommandConfirmationResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a VoiceCommandConfirmationResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VoiceCommandConfirmationResult {}
unsafe impl ::std::marker::Sync for VoiceCommandConfirmationResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct VoiceCommandContentTile(::windows::runtime::IInspectable);
impl VoiceCommandContentTile {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            VoiceCommandContentTile,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TextLine1(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetTextLine1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TextLine2(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetTextLine2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn TextLine3(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetTextLine3<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Storage")]
    pub fn Image(&self) -> ::windows::runtime::Result<super::super::Storage::IStorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::IStorageFile>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    pub fn SetImage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFile>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AppContext(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn SetAppContext<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AppLaunchArgument(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAppLaunchArgument<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ContentTileType(&self) -> ::windows::runtime::Result<VoiceCommandContentTileType> {
        let this = self;
        unsafe {
            let mut result__: VoiceCommandContentTileType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<VoiceCommandContentTileType>(result__)
        }
    }
    pub fn SetContentTileType(
        &self,
        value: VoiceCommandContentTileType,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommandContentTile {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTile;{3eefe9f0-b8c7-4c76-a0de-1607895ee327})" ) ;
}
unsafe impl ::windows::runtime::Interface for VoiceCommandContentTile {
    type Vtable = IVoiceCommandContentTile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1055910384,
        47303,
        19574,
        [160, 222, 22, 7, 137, 94, 227, 39],
    );
}
impl ::windows::runtime::RuntimeName for VoiceCommandContentTile {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTile";
}
impl ::std::convert::From<VoiceCommandContentTile> for ::windows::runtime::IUnknown {
    fn from(value: VoiceCommandContentTile) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VoiceCommandContentTile> for ::windows::runtime::IUnknown {
    fn from(value: &VoiceCommandContentTile) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for VoiceCommandContentTile
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &VoiceCommandContentTile
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<VoiceCommandContentTile> for ::windows::runtime::IInspectable {
    fn from(value: VoiceCommandContentTile) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceCommandContentTile> for ::windows::runtime::IInspectable {
    fn from(value: &VoiceCommandContentTile) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for VoiceCommandContentTile
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a VoiceCommandContentTile
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VoiceCommandContentTile {}
unsafe impl ::std::marker::Sync for VoiceCommandContentTile {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VoiceCommandContentTileType(pub i32);
impl VoiceCommandContentTileType {
    pub const TitleOnly: VoiceCommandContentTileType = VoiceCommandContentTileType(0i32);
    pub const TitleWithText: VoiceCommandContentTileType = VoiceCommandContentTileType(1i32);
    pub const TitleWith68x68Icon: VoiceCommandContentTileType = VoiceCommandContentTileType(2i32);
    pub const TitleWith68x68IconAndText: VoiceCommandContentTileType =
        VoiceCommandContentTileType(3i32);
    pub const TitleWith68x92Icon: VoiceCommandContentTileType = VoiceCommandContentTileType(4i32);
    pub const TitleWith68x92IconAndText: VoiceCommandContentTileType =
        VoiceCommandContentTileType(5i32);
    pub const TitleWith280x140Icon: VoiceCommandContentTileType = VoiceCommandContentTileType(6i32);
    pub const TitleWith280x140IconAndText: VoiceCommandContentTileType =
        VoiceCommandContentTileType(7i32);
}
impl ::std::convert::From<i32> for VoiceCommandContentTileType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VoiceCommandContentTileType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommandContentTileType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.VoiceCommands.VoiceCommandContentTileType;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct VoiceCommandDefinition(::windows::runtime::IInspectable);
impl VoiceCommandDefinition {
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SetPhraseListAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        &self,
        phraselistname: Param0,
        phraselist: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                phraselistname.into_param().abi(),
                phraselist.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommandDefinition {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition;{7972aad0-0974-4979-984b-cb8959cd61ae})" ) ;
}
unsafe impl ::windows::runtime::Interface for VoiceCommandDefinition {
    type Vtable = IVoiceCommandDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2037557968,
        2420,
        18809,
        [152, 75, 203, 137, 89, 205, 97, 174],
    );
}
impl ::windows::runtime::RuntimeName for VoiceCommandDefinition {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinition";
}
impl ::std::convert::From<VoiceCommandDefinition> for ::windows::runtime::IUnknown {
    fn from(value: VoiceCommandDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VoiceCommandDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &VoiceCommandDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for VoiceCommandDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &VoiceCommandDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<VoiceCommandDefinition> for ::windows::runtime::IInspectable {
    fn from(value: VoiceCommandDefinition) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceCommandDefinition> for ::windows::runtime::IInspectable {
    fn from(value: &VoiceCommandDefinition) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for VoiceCommandDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a VoiceCommandDefinition
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VoiceCommandDefinition {}
unsafe impl ::std::marker::Sync for VoiceCommandDefinition {}
pub struct VoiceCommandDefinitionManager {}
impl VoiceCommandDefinitionManager {
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn InstallCommandDefinitionsFromStorageFileAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>,
    >(
        file: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IVoiceCommandDefinitionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                file.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn InstalledCommandDefinitions() -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMapView<
            ::windows::runtime::HSTRING,
            VoiceCommandDefinition,
        >,
    > {
        Self::IVoiceCommandDefinitionManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMapView<
                ::windows::runtime::HSTRING,
                VoiceCommandDefinition,
            >>(result__)
        })
    }
    pub fn IVoiceCommandDefinitionManagerStatics<
        R,
        F: FnOnce(&IVoiceCommandDefinitionManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            VoiceCommandDefinitionManager,
            IVoiceCommandDefinitionManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for VoiceCommandDefinitionManager {
    const NAME: &'static str =
        "Windows.ApplicationModel.VoiceCommands.VoiceCommandDefinitionManager";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct VoiceCommandDisambiguationResult(::windows::runtime::IInspectable);
impl VoiceCommandDisambiguationResult {
    pub fn SelectedItem(&self) -> ::windows::runtime::Result<VoiceCommandContentTile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<VoiceCommandContentTile>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommandDisambiguationResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandDisambiguationResult;{ecc68cfe-c9ac-45df-a8ea-feea08ef9c5e})" ) ;
}
unsafe impl ::windows::runtime::Interface for VoiceCommandDisambiguationResult {
    type Vtable = IVoiceCommandDisambiguationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3972435198,
        51628,
        17887,
        [168, 234, 254, 234, 8, 239, 156, 94],
    );
}
impl ::windows::runtime::RuntimeName for VoiceCommandDisambiguationResult {
    const NAME: &'static str =
        "Windows.ApplicationModel.VoiceCommands.VoiceCommandDisambiguationResult";
}
impl ::std::convert::From<VoiceCommandDisambiguationResult> for ::windows::runtime::IUnknown {
    fn from(value: VoiceCommandDisambiguationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VoiceCommandDisambiguationResult> for ::windows::runtime::IUnknown {
    fn from(value: &VoiceCommandDisambiguationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for VoiceCommandDisambiguationResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &VoiceCommandDisambiguationResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<VoiceCommandDisambiguationResult> for ::windows::runtime::IInspectable {
    fn from(value: VoiceCommandDisambiguationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceCommandDisambiguationResult> for ::windows::runtime::IInspectable {
    fn from(value: &VoiceCommandDisambiguationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for VoiceCommandDisambiguationResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a VoiceCommandDisambiguationResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VoiceCommandDisambiguationResult {}
unsafe impl ::std::marker::Sync for VoiceCommandDisambiguationResult {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct VoiceCommandResponse(::windows::runtime::IInspectable);
impl VoiceCommandResponse {
    pub fn Message(&self) -> ::windows::runtime::Result<VoiceCommandUserMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<VoiceCommandUserMessage>(result__)
        }
    }
    pub fn SetMessage<'a, Param0: ::windows::runtime::IntoParam<'a, VoiceCommandUserMessage>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RepeatMessage(&self) -> ::windows::runtime::Result<VoiceCommandUserMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<VoiceCommandUserMessage>(result__)
        }
    }
    pub fn SetRepeatMessage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, VoiceCommandUserMessage>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn AppLaunchArgument(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetAppLaunchArgument<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn VoiceCommandContentTiles(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVector<VoiceCommandContentTile>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVector<VoiceCommandContentTile>>(
                result__,
            )
        }
    }
    pub fn MaxSupportedVoiceCommandContentTiles() -> ::windows::runtime::Result<u32> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        })
    }
    pub fn CreateResponse<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, VoiceCommandUserMessage>,
    >(
        usermessage: Param0,
    ) -> ::windows::runtime::Result<VoiceCommandResponse> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                usermessage.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VoiceCommandResponse>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateResponseWithTiles<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, VoiceCommandUserMessage>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>,
        >,
    >(
        message: Param0,
        contenttiles: Param1,
    ) -> ::windows::runtime::Result<VoiceCommandResponse> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                message.into_param().abi(),
                contenttiles.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VoiceCommandResponse>(result__)
        })
    }
    pub fn CreateResponseForPrompt<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, VoiceCommandUserMessage>,
        Param1: ::windows::runtime::IntoParam<'a, VoiceCommandUserMessage>,
    >(
        message: Param0,
        repeatmessage: Param1,
    ) -> ::windows::runtime::Result<VoiceCommandResponse> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                message.into_param().abi(),
                repeatmessage.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VoiceCommandResponse>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateResponseForPromptWithTiles<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, VoiceCommandUserMessage>,
        Param1: ::windows::runtime::IntoParam<'a, VoiceCommandUserMessage>,
        Param2: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<VoiceCommandContentTile>,
        >,
    >(
        message: Param0,
        repeatmessage: Param1,
        contenttiles: Param2,
    ) -> ::windows::runtime::Result<VoiceCommandResponse> {
        Self::IVoiceCommandResponseStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                message.into_param().abi(),
                repeatmessage.into_param().abi(),
                contenttiles.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VoiceCommandResponse>(result__)
        })
    }
    pub fn IVoiceCommandResponseStatics<
        R,
        F: FnOnce(&IVoiceCommandResponseStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            VoiceCommandResponse,
            IVoiceCommandResponseStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommandResponse {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandResponse;{0284b30e-8a3b-4cc4-a6a1-cad5be2716b5})" ) ;
}
unsafe impl ::windows::runtime::Interface for VoiceCommandResponse {
    type Vtable = IVoiceCommandResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        42251022,
        35387,
        19652,
        [166, 161, 202, 213, 190, 39, 22, 181],
    );
}
impl ::windows::runtime::RuntimeName for VoiceCommandResponse {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandResponse";
}
impl ::std::convert::From<VoiceCommandResponse> for ::windows::runtime::IUnknown {
    fn from(value: VoiceCommandResponse) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VoiceCommandResponse> for ::windows::runtime::IUnknown {
    fn from(value: &VoiceCommandResponse) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for VoiceCommandResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &VoiceCommandResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<VoiceCommandResponse> for ::windows::runtime::IInspectable {
    fn from(value: VoiceCommandResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceCommandResponse> for ::windows::runtime::IInspectable {
    fn from(value: &VoiceCommandResponse) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for VoiceCommandResponse
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a VoiceCommandResponse
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VoiceCommandResponse {}
unsafe impl ::std::marker::Sync for VoiceCommandResponse {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct VoiceCommandServiceConnection(::windows::runtime::IInspectable);
impl VoiceCommandServiceConnection {
    #[cfg(feature = "Foundation")]
    pub fn GetVoiceCommandAsync(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<VoiceCommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<VoiceCommand>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestConfirmationAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, VoiceCommandResponse>,
    >(
        &self,
        response: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<VoiceCommandConfirmationResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                response.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<VoiceCommandConfirmationResult>>(
                result__,
            )
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestDisambiguationAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, VoiceCommandResponse>,
    >(
        &self,
        response: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<VoiceCommandDisambiguationResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .8 ) ( :: std :: mem :: transmute_copy ( this ) , response . into_param ( ) . abi ( ) , & mut result__ ) . from_abi :: < super::super::Foundation:: IAsyncOperation :: < VoiceCommandDisambiguationResult > > ( result__ )
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportProgressAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, VoiceCommandResponse>,
    >(
        &self,
        response: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                response.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportSuccessAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, VoiceCommandResponse>,
    >(
        &self,
        response: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                response.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ReportFailureAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, VoiceCommandResponse>,
    >(
        &self,
        response: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                response.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAppLaunchAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, VoiceCommandResponse>,
    >(
        &self,
        response: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                response.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Globalization")]
    pub fn Language(&self) -> ::windows::runtime::Result<super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Globalization::Language>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn VoiceCommandCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                VoiceCommandServiceConnection,
                VoiceCommandCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveVoiceCommandCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn FromAppServiceTriggerDetails<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::AppService::AppServiceTriggerDetails>,
    >(
        triggerdetails: Param0,
    ) -> ::windows::runtime::Result<VoiceCommandServiceConnection> {
        Self::IVoiceCommandServiceConnectionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                triggerdetails.into_param().abi(),
                &mut result__,
            )
            .from_abi::<VoiceCommandServiceConnection>(result__)
        })
    }
    pub fn IVoiceCommandServiceConnectionStatics<
        R,
        F: FnOnce(&IVoiceCommandServiceConnectionStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            VoiceCommandServiceConnection,
            IVoiceCommandServiceConnectionStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommandServiceConnection {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandServiceConnection;{d894bb9f-21da-44a4-98a2-fb131920a9cc})" ) ;
}
unsafe impl ::windows::runtime::Interface for VoiceCommandServiceConnection {
    type Vtable = IVoiceCommandServiceConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3633626015,
        8666,
        17572,
        [152, 162, 251, 19, 25, 32, 169, 204],
    );
}
impl ::windows::runtime::RuntimeName for VoiceCommandServiceConnection {
    const NAME: &'static str =
        "Windows.ApplicationModel.VoiceCommands.VoiceCommandServiceConnection";
}
impl ::std::convert::From<VoiceCommandServiceConnection> for ::windows::runtime::IUnknown {
    fn from(value: VoiceCommandServiceConnection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VoiceCommandServiceConnection> for ::windows::runtime::IUnknown {
    fn from(value: &VoiceCommandServiceConnection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for VoiceCommandServiceConnection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &VoiceCommandServiceConnection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<VoiceCommandServiceConnection> for ::windows::runtime::IInspectable {
    fn from(value: VoiceCommandServiceConnection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceCommandServiceConnection> for ::windows::runtime::IInspectable {
    fn from(value: &VoiceCommandServiceConnection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for VoiceCommandServiceConnection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a VoiceCommandServiceConnection
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VoiceCommandServiceConnection {}
unsafe impl ::std::marker::Sync for VoiceCommandServiceConnection {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct VoiceCommandUserMessage(::windows::runtime::IInspectable);
impl VoiceCommandUserMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            VoiceCommandUserMessage,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn DisplayMessage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetDisplayMessage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn SpokenMessage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetSpokenMessage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for VoiceCommandUserMessage {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.ApplicationModel.VoiceCommands.VoiceCommandUserMessage;{674eb3c0-44f6-4f07-b979-4c723fc08597})" ) ;
}
unsafe impl ::windows::runtime::Interface for VoiceCommandUserMessage {
    type Vtable = IVoiceCommandUserMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1733211072,
        17654,
        20231,
        [185, 121, 76, 114, 63, 192, 133, 151],
    );
}
impl ::windows::runtime::RuntimeName for VoiceCommandUserMessage {
    const NAME: &'static str = "Windows.ApplicationModel.VoiceCommands.VoiceCommandUserMessage";
}
impl ::std::convert::From<VoiceCommandUserMessage> for ::windows::runtime::IUnknown {
    fn from(value: VoiceCommandUserMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&VoiceCommandUserMessage> for ::windows::runtime::IUnknown {
    fn from(value: &VoiceCommandUserMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for VoiceCommandUserMessage
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &VoiceCommandUserMessage
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<VoiceCommandUserMessage> for ::windows::runtime::IInspectable {
    fn from(value: VoiceCommandUserMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&VoiceCommandUserMessage> for ::windows::runtime::IInspectable {
    fn from(value: &VoiceCommandUserMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for VoiceCommandUserMessage
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a VoiceCommandUserMessage
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for VoiceCommandUserMessage {}
unsafe impl ::std::marker::Sync for VoiceCommandUserMessage {}
