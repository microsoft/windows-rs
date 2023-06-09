#[cfg(feature = "Media_Core_Preview")]
pub mod Preview;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStreamDescriptor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioStreamDescriptor {
    type Vtable = IAudioStreamDescriptor_Vtbl;
}
impl ::core::clone::Clone for IAudioStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioStreamDescriptor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1e3692e4_4027_4847_a70b_df1d9a2a7b04);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStreamDescriptor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioStreamDescriptor2 {
    type Vtable = IAudioStreamDescriptor2_Vtbl;
}
impl ::core::clone::Clone for IAudioStreamDescriptor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioStreamDescriptor2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e68f1f6_a448_497b_8840_85082665acf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamDescriptor2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetLeadingEncoderPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLeadingEncoderPadding: usize,
    #[cfg(feature = "Foundation")]
    pub LeadingEncoderPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LeadingEncoderPadding: usize,
    #[cfg(feature = "Foundation")]
    pub SetTrailingEncoderPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTrailingEncoderPadding: usize,
    #[cfg(feature = "Foundation")]
    pub TrailingEncoderPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrailingEncoderPadding: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStreamDescriptor3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioStreamDescriptor3 {
    type Vtable = IAudioStreamDescriptor3_Vtbl;
}
impl ::core::clone::Clone for IAudioStreamDescriptor3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioStreamDescriptor3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4d220da1_8e83_44ef_8973_2f63e993f36b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamDescriptor3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioStreamDescriptorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioStreamDescriptorFactory {
    type Vtable = IAudioStreamDescriptorFactory_Vtbl;
}
impl ::core::clone::Clone for IAudioStreamDescriptorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioStreamDescriptorFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4a86ce9e_4cb1_4380_8e0c_83504b7f5bf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamDescriptorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioTrack(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioTrack {
    type Vtable = IAudioTrack_Vtbl;
}
impl ::core::clone::Clone for IAudioTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioTrack {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf23b6e77_3ef7_40de_b943_068b1321701d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTrack_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub OpenFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpenFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpenFailed: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetEncodingProperties: usize,
    #[cfg(feature = "Media_Playback")]
    pub PlaybackItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SupportInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioTrackOpenFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioTrackOpenFailedEventArgs {
    type Vtable = IAudioTrackOpenFailedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IAudioTrackOpenFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioTrackOpenFailedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeeddb9b9_bb7c_4112_bf76_9384676f824b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTrackOpenFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAudioTrackSupportInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAudioTrackSupportInfo {
    type Vtable = IAudioTrackSupportInfo_Vtbl;
}
impl ::core::clone::Clone for IAudioTrackSupportInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAudioTrackSupportInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x178beff7_cc39_44a6_b951_4a5653f073fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTrackSupportInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DecoderStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaDecoderStatus) -> ::windows_core::HRESULT,
    pub Degradation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDecoderDegradation) -> ::windows_core::HRESULT,
    pub DegradationReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AudioDecoderDegradationReason) -> ::windows_core::HRESULT,
    pub MediaSourceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IChapterCue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IChapterCue {
    type Vtable = IChapterCue_Vtbl;
}
impl ::core::clone::Clone for IChapterCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IChapterCue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72a98001_d38a_4c0a_8fa6_75cddaf4664c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChapterCue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICodecInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICodecInfo {
    type Vtable = ICodecInfo_Vtbl;
}
impl ::core::clone::Clone for ICodecInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICodecInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51e89f85_ea97_499c_86ac_4ce5e73f3a42);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICodecInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CodecKind) -> ::windows_core::HRESULT,
    pub Category: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CodecCategory) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Subtypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Subtypes: usize,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsTrusted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICodecQuery(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICodecQuery {
    type Vtable = ICodecQuery_Vtbl;
}
impl ::core::clone::Clone for ICodecQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICodecQuery {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x222a953a_af61_4e04_808a_a4634e2f3ac4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICodecQuery_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: CodecKind, category: CodecCategory, subtype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICodecSubtypesStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ICodecSubtypesStatics {
    type Vtable = ICodecSubtypesStatics_Vtbl;
}
impl ::core::clone::Clone for ICodecSubtypesStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICodecSubtypesStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa66ac4f2_888b_4224_8cf6_2a8d4eb02382);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICodecSubtypesStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub VideoFormatDV25: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDV50: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvh1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvhD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvsd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatDvsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatH263: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatH264: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatH265: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatH264ES: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatHevc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatHevcES: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatM4S2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMjpg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMP43: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMP4S: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMP4V: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMpeg2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatVP80: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatVP90: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMpg1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMss1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatMss2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatWmv1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatWmv2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatWmv3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormatWvc1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub VideoFormat420O: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAdts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAlac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAmrNB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAmrWB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatAmrWP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDolbyAC3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDolbyAC3Spdif: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDolbyDDPlus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDrm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatDts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatFlac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatFloat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatMP3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatMPeg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatMsp1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatOpus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatPcm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatWmaSpdif: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatWMAudioLossless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatWMAudioV8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AudioFormatWMAudioV9: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataCue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataCue {
    type Vtable = IDataCue_Vtbl;
}
impl ::core::clone::Clone for IDataCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDataCue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7c7f676d_1fbc_4e2d_9a87_ee38bd1dc637);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub SetData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDataCue2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDataCue2 {
    type Vtable = IDataCue2_Vtbl;
}
impl ::core::clone::Clone for IDataCue2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDataCue2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc561b15_95f2_49e8_96f1_8dd5dac68d93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataCue2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceDetectedEventArgs {
    type Vtable = IFaceDetectedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IFaceDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFaceDetectedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x19918426_c65b_46ba_85f8_13880576c90a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ResultFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectionEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceDetectionEffect {
    type Vtable = IFaceDetectionEffect_Vtbl;
}
impl ::core::clone::Clone for IFaceDetectionEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFaceDetectionEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae15ebd2_0542_42a9_bc90_f283a29f46c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectionEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDesiredDetectionInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredDetectionInterval: usize,
    #[cfg(feature = "Foundation")]
    pub DesiredDetectionInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredDetectionInterval: usize,
    #[cfg(feature = "Foundation")]
    pub FaceDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FaceDetected: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFaceDetected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFaceDetected: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectionEffectDefinition(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceDetectionEffectDefinition {
    type Vtable = IFaceDetectionEffectDefinition_Vtbl;
}
impl ::core::clone::Clone for IFaceDetectionEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFaceDetectionEffectDefinition {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x43dca081_b848_4f33_b702_1fd2624fb016);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectionEffectDefinition_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetDetectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FaceDetectionMode) -> ::windows_core::HRESULT,
    pub DetectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FaceDetectionMode) -> ::windows_core::HRESULT,
    pub SetSynchronousDetectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub SynchronousDetectionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFaceDetectionEffectFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IFaceDetectionEffectFrame {
    type Vtable = IFaceDetectionEffectFrame_Vtbl;
}
impl ::core::clone::Clone for IFaceDetectionEffectFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IFaceDetectionEffectFrame {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8ab08993_5dc8_447b_a247_5270bd802ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFaceDetectionEffectFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis"))]
    pub DetectedFaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis")))]
    DetectedFaces: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHighDynamicRangeControl(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHighDynamicRangeControl {
    type Vtable = IHighDynamicRangeControl_Vtbl;
}
impl ::core::clone::Clone for IHighDynamicRangeControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHighDynamicRangeControl {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55f1a7ae_d957_4dc9_9d1c_8553a82a7d99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHighDynamicRangeControl_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHighDynamicRangeOutput(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IHighDynamicRangeOutput {
    type Vtable = IHighDynamicRangeOutput_Vtbl;
}
impl ::core::clone::Clone for IHighDynamicRangeOutput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IHighDynamicRangeOutput {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f57806b_253b_4119_bb40_3a90e51384f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHighDynamicRangeOutput_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Certainty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Devices_Core"))]
    pub FrameControllers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Media_Devices_Core")))]
    FrameControllers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IImageCue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IImageCue {
    type Vtable = IImageCue_Vtbl;
}
impl ::core::clone::Clone for IImageCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IImageCue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x52828282_367b_440b_9116_3c84570dd270);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageCue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPoint) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextPoint) -> ::windows_core::HRESULT,
    pub Extent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextSize) -> ::windows_core::HRESULT,
    pub SetExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextSize) -> ::windows_core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetSoftwareBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SoftwareBitmap: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInitializeMediaStreamSourceRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IInitializeMediaStreamSourceRequestedEventArgs {
    type Vtable = IInitializeMediaStreamSourceRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IInitializeMediaStreamSourceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IInitializeMediaStreamSourceRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x25bc45e1_9b08_4c2e_a855_4542f1a75deb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeMediaStreamSourceRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub RandomAccessStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    RandomAccessStream: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLightFusionResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLightFusionResult {
    type Vtable = ILowLightFusionResult_Vtbl;
}
impl ::core::clone::Clone for ILowLightFusionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILowLightFusionResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x78edbe35_27a0_42e0_9cd3_738d2089de9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLightFusionResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    Frame: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILowLightFusionStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILowLightFusionStatics {
    type Vtable = ILowLightFusionStatics_Vtbl;
}
impl ::core::clone::Clone for ILowLightFusionStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILowLightFusionStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5305016d_c29e_40e2_87a9_9e1fd2f192f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILowLightFusionStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub SupportedBitmapPixelFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    SupportedBitmapPixelFormats: usize,
    pub MaxSupportedFrameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub FuseAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameset: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Graphics_Imaging")))]
    FuseAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBinder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBinder {
    type Vtable = IMediaBinder_Vtbl;
}
impl ::core::clone::Clone for IMediaBinder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaBinder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b7e40aa_de07_424f_83f1_f1de46c4fa2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBinder_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Binding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Binding: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBinding: usize,
    pub Token: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBindingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBindingEventArgs {
    type Vtable = IMediaBindingEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaBindingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaBindingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb61cb25a_1b6d_4630_a86d_2f0837f712e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBindingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Canceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Canceled: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCanceled: usize,
    pub MediaBinder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, contenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStreamReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, contenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStreamReference: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBindingEventArgs2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBindingEventArgs2 {
    type Vtable = IMediaBindingEventArgs2_Vtbl;
}
impl ::core::clone::Clone for IMediaBindingEventArgs2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaBindingEventArgs2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0464cceb_bb5a_482f_b8ba_f0284c696567);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBindingEventArgs2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub SetAdaptiveMediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Streaming_Adaptive"))]
    SetAdaptiveMediaSource: usize,
    #[cfg(feature = "Storage")]
    pub SetStorageFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetStorageFile: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaBindingEventArgs3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaBindingEventArgs3 {
    type Vtable = IMediaBindingEventArgs3_Vtbl;
}
impl ::core::clone::Clone for IMediaBindingEventArgs3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaBindingEventArgs3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8eb475e_19be_44fc_a5ed_7aba315037f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaBindingEventArgs3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub SetDownloadOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadoperation: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))]
    SetDownloadOperation: usize,
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct IMediaCue(::windows_core::IUnknown);
impl IMediaCue {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IMediaCue, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IMediaCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaCue {}
impl ::core::fmt::Debug for IMediaCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaCue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IMediaCue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{c7d15e5d-59dc-431f-a0ee-27744323b36d}");
}
unsafe impl ::windows_core::Interface for IMediaCue {
    type Vtable = IMediaCue_Vtbl;
}
impl ::core::clone::Clone for IMediaCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc7d15e5d_59dc_431f_a0ee_27744323b36d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetStartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartTime: usize,
    #[cfg(feature = "Foundation")]
    pub StartTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaCueEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaCueEventArgs {
    type Vtable = IMediaCueEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaCueEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaCueEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd12f47f7_5fa4_4e68_9fe5_32160dcee57e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCueEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Cue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct IMediaSource(::windows_core::IUnknown);
impl IMediaSource {}
::windows_core::imp::interface_hierarchy!(IMediaSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IMediaSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaSource {}
impl ::core::fmt::Debug for IMediaSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IMediaSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{e7bfb599-a09d-4c21-bcdf-20af4f86b3d9}");
}
unsafe impl ::windows_core::Interface for IMediaSource {
    type Vtable = IMediaSource_Vtbl;
}
impl ::core::clone::Clone for IMediaSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7bfb599_a09d_4c21_bcdf_20af4f86b3d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSource2 {
    type Vtable = IMediaSource2_Vtbl;
}
impl ::core::clone::Clone for IMediaSource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSource2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2eb61048_655f_4c37_b813_b4e45dfa0abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub OpenOperationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenOperationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpenOperationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpenOperationCompleted: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CustomProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CustomProperties: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ExternalTimedTextSources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExternalTimedTextSources: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ExternalTimedMetadataTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExternalTimedMetadataTracks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSource3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSource3 {
    type Vtable = IMediaSource3_Vtbl;
}
impl ::core::clone::Clone for IMediaSource3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSource3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb59f0d9b_4b6e_41ed_bbb4_7c7509a994ad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StateChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStateChanged: usize,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSource4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSource4 {
    type Vtable = IMediaSource4_Vtbl;
}
impl ::core::clone::Clone for IMediaSource4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSource4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbdafad57_8eff_4c63_85a6_84de0ae3e4f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub AdaptiveMediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Streaming_Adaptive"))]
    AdaptiveMediaSource: usize,
    pub MediaStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MseStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub OpenAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSource5(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSource5 {
    type Vtable = IMediaSource5_Vtbl;
}
impl ::core::clone::Clone for IMediaSource5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSource5 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x331a22ae_ed2e_4a22_94c8_b743a92b3022);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource5_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub DownloadOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))]
    DownloadOperation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceAppServiceConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceAppServiceConnection {
    type Vtable = IMediaSourceAppServiceConnection_Vtbl;
}
impl ::core::clone::Clone for IMediaSourceAppServiceConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSourceAppServiceConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x61e1ea97_1916_4810_b7f4_b642be829596);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceAppServiceConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub InitializeMediaStreamSourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InitializeMediaStreamSourceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveInitializeMediaStreamSourceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveInitializeMediaStreamSourceRequested: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceAppServiceConnectionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceAppServiceConnectionFactory {
    type Vtable = IMediaSourceAppServiceConnectionFactory_Vtbl;
}
impl ::core::clone::Clone for IMediaSourceAppServiceConnectionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSourceAppServiceConnectionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x65b912eb_80b9_44f9_9c1e_e120f6d92838);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceAppServiceConnectionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "ApplicationModel_AppService")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appserviceconnection: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceError(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceError {
    type Vtable = IMediaSourceError_Vtbl;
}
impl ::core::clone::Clone for IMediaSourceError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSourceError {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c0a8965_37c5_4e9d_8d21_1cdee90cecc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceError_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceOpenOperationCompletedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceOpenOperationCompletedEventArgs {
    type Vtable = IMediaSourceOpenOperationCompletedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaSourceOpenOperationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSourceOpenOperationCompletedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfc682ceb_e281_477c_a8e0_1acd654114c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceOpenOperationCompletedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStateChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceStateChangedEventArgs {
    type Vtable = IMediaSourceStateChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaSourceStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSourceStateChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a30af82_9071_4bac_bc39_ca2a93b717a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStateChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OldState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows_core::HRESULT,
    pub NewState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceState) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceStatics {
    type Vtable = IMediaSourceStatics_Vtbl;
}
impl ::core::clone::Clone for IMediaSourceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSourceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf77d6fa4_4652_410e_b1d8_e9a5e245a45c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub CreateFromAdaptiveMediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Streaming_Adaptive"))]
    CreateFromAdaptiveMediaSource: usize,
    pub CreateFromMediaStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFromMseStreamSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFromIMediaSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mediasource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub CreateFromStorageFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    CreateFromStorageFile: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, contenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, contenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamReference: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFromUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceStatics2 {
    type Vtable = IMediaSourceStatics2_Vtbl;
}
impl ::core::clone::Clone for IMediaSourceStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSourceStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeee161a4_7f13_4896_b8cb_df0de5bcb9f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromMediaBinder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, binder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceStatics3 {
    type Vtable = IMediaSourceStatics3_Vtbl;
}
impl ::core::clone::Clone for IMediaSourceStatics3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSourceStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x453a30d6_2bea_4122_9f73_eace04526e35);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Capture_Frames")]
    pub CreateFromMediaFrameSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, framesource: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture_Frames"))]
    CreateFromMediaFrameSource: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaSourceStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaSourceStatics4 {
    type Vtable = IMediaSourceStatics4_Vtbl;
}
impl ::core::clone::Clone for IMediaSourceStatics4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaSourceStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x281b3bfc_e50a_4428_a500_9c4ed918d3f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub CreateFromDownloadOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, downloadoperation: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Networking_BackgroundTransfer"))]
    CreateFromDownloadOperation: usize,
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct IMediaStreamDescriptor(::windows_core::IUnknown);
impl IMediaStreamDescriptor {
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IMediaStreamDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IMediaStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaStreamDescriptor {}
impl ::core::fmt::Debug for IMediaStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaStreamDescriptor").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IMediaStreamDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{80f16e6e-92f7-451e-97d2-afd80742da70}");
}
unsafe impl ::windows_core::Interface for IMediaStreamDescriptor {
    type Vtable = IMediaStreamDescriptor_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamDescriptor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x80f16e6e_92f7_451e_97d2_afd80742da70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct IMediaStreamDescriptor2(::windows_core::IUnknown);
impl IMediaStreamDescriptor2 {
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IMediaStreamDescriptor2, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaStreamDescriptor> for IMediaStreamDescriptor2 {}
impl ::core::cmp::PartialEq for IMediaStreamDescriptor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaStreamDescriptor2 {}
impl ::core::fmt::Debug for IMediaStreamDescriptor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaStreamDescriptor2").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IMediaStreamDescriptor2 {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{5073010f-e8b2-4071-b00b-ebf337a76b58}");
}
unsafe impl ::windows_core::Interface for IMediaStreamDescriptor2 {
    type Vtable = IMediaStreamDescriptor2_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamDescriptor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamDescriptor2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5073010f_e8b2_4071_b00b_ebf337a76b58);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamDescriptor2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSample(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSample {
    type Vtable = IMediaStreamSample_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSample {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSample {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c8db627_4b80_4361_9837_6cb7481ad9d6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSample_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Processed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Processed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveProcessed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveProcessed: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Buffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Buffer: usize,
    #[cfg(feature = "Foundation")]
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Timestamp: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ExtendedProperties: usize,
    pub Protection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDecodeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDecodeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub DecodeTimestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DecodeTimestamp: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub SetKeyFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub KeyFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDiscontinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Discontinuous: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSample2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSample2 {
    type Vtable = IMediaStreamSample2_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSample2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSample2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x45078691_fce8_4746_a1c8_10c25d3d7cd3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSample2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub Direct3D11Surface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    Direct3D11Surface: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSampleProtectionProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSampleProtectionProperties {
    type Vtable = IMediaStreamSampleProtectionProperties_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSampleProtectionProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSampleProtectionProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4eb88292_ecdf_493e_841d_dd4add7caca2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSampleProtectionProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetKeyIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    pub GetKeyIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_core::HRESULT,
    pub SetInitializationVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    pub GetInitializationVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_core::HRESULT,
    pub SetSubSampleMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8) -> ::windows_core::HRESULT,
    pub GetSubSampleMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSampleStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSampleStatics {
    type Vtable = IMediaStreamSampleStatics_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSampleStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSampleStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfdf218f_a6cf_4579_be41_73dd941ad972);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSampleStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, timestamp: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateFromBuffer: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateFromStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, count: u32, timestamp: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateFromStreamAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSampleStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSampleStatics2 {
    type Vtable = IMediaStreamSampleStatics2_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSampleStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSampleStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9efe9521_6d46_494c_a2f8_d662922e2dd7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSampleStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateFromDirect3D11Surface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: *mut ::core::ffi::c_void, timestamp: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CreateFromDirect3D11Surface: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSource {
    type Vtable = IMediaStreamSource_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3712d543_45eb_4138_aa62_c01e26f3843f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(feature = "Foundation")]
    pub Starting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Starting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStarting: usize,
    #[cfg(feature = "Foundation")]
    pub Paused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Paused: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaused: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaused: usize,
    #[cfg(feature = "Foundation")]
    pub SampleRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SampleRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSampleRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSampleRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SwitchStreamsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SwitchStreamsRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSwitchStreamsRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSwitchStreamsRequested: usize,
    pub NotifyError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errorstatus: MediaStreamSourceErrorStatus) -> ::windows_core::HRESULT,
    pub AddStreamDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Media_Protection")]
    pub SetMediaProtectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    SetMediaProtectionManager: usize,
    #[cfg(feature = "Media_Protection")]
    pub MediaProtectionManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Protection"))]
    MediaProtectionManager: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    pub SetCanSeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub CanSeek: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetBufferTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBufferTime: usize,
    #[cfg(feature = "Foundation")]
    pub BufferTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BufferTime: usize,
    #[cfg(feature = "Foundation")]
    pub SetBufferedRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startoffset: super::super::Foundation::TimeSpan, endoffset: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBufferedRange: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub MusicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    MusicProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub VideoProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    VideoProperties: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetThumbnail: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    pub AddProtectionKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamdescriptor: *mut ::core::ffi::c_void, keyIdentifier_array_size: u32, keyidentifier: *const u8, licenseData_array_size: u32, licensedata: *const u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSource2 {
    type Vtable = IMediaStreamSource2_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSource2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec55d0ad_2e6a_4f74_adbb_b562d1533849);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSource2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SampleRendered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SampleRendered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSampleRendered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSampleRendered: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSource3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSource3 {
    type Vtable = IMediaStreamSource3_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSource3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSource3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a2a2746_3ddd_4ddf_a121_94045ecf9440);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSource3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SetMaxSupportedPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetMaxSupportedPlaybackRate: usize,
    #[cfg(feature = "Foundation")]
    pub MaxSupportedPlaybackRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxSupportedPlaybackRate: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSource4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSource4 {
    type Vtable = IMediaStreamSource4_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSource4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSource4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1d0cfcab_830d_417c_a3a9_2454fd6415c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSource4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetIsLive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsLive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceClosedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceClosedEventArgs {
    type Vtable = IMediaStreamSourceClosedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceClosedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd8c7eb2_4816_4e24_88f0_491ef7386406);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceClosedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceClosedRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceClosedRequest {
    type Vtable = IMediaStreamSourceClosedRequest_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceClosedRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceClosedRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x907c00e9_18a3_4951_887a_2c1eebd5c69e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceClosedRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaStreamSourceClosedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceFactory {
    type Vtable = IMediaStreamSourceFactory_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef77e0d9_d158_4b7a_863f_203342fbfd41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateFromDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateFromDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: *mut ::core::ffi::c_void, descriptor2: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRenderedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSampleRenderedEventArgs {
    type Vtable = IMediaStreamSourceSampleRenderedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceSampleRenderedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceSampleRenderedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d697b05_d4f2_4c7a_9dfe_8d6cd0b3ee84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSampleRenderedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SampleLag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SampleLag: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSampleRequest {
    type Vtable = IMediaStreamSourceSampleRequest_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceSampleRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceSampleRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4db341a9_3501_4d9b_83f9_8f235c822532);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSampleRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StreamDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Sample: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportSampleProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progress: u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequestDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSampleRequestDeferral {
    type Vtable = IMediaStreamSourceSampleRequestDeferral_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceSampleRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceSampleRequestDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7895cc02_f982_43c8_9d16_c62d999319be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSampleRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSampleRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSampleRequestedEventArgs {
    type Vtable = IMediaStreamSourceSampleRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceSampleRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceSampleRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10f9bb9e_71c5_492f_847f_0da1f35e81f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSampleRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceStartingEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceStartingEventArgs {
    type Vtable = IMediaStreamSourceStartingEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceStartingEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf41468f2_c274_4940_a5bb_28a572452fa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceStartingEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceStartingRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceStartingRequest {
    type Vtable = IMediaStreamSourceStartingRequest_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceStartingRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceStartingRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2a9093e4_35c4_4b1b_a791_0d99db56dd1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceStartingRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPosition: usize,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetActualStartPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActualStartPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceStartingRequestDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceStartingRequestDeferral {
    type Vtable = IMediaStreamSourceStartingRequestDeferral_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceStartingRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceStartingRequestDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3f1356a5_6340_4dc4_9910_068ed9f598f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceStartingRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequest(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSwitchStreamsRequest {
    type Vtable = IMediaStreamSourceSwitchStreamsRequest_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceSwitchStreamsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceSwitchStreamsRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41b8808e_38a9_4ec3_9ba0_b69b85501e90);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSwitchStreamsRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub OldStreamDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub NewStreamDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequestDeferral(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSwitchStreamsRequestDeferral {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestDeferral_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceSwitchStreamsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceSwitchStreamsRequestDeferral {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbee3d835_a505_4f9a_b943_2b8cb1b4bbd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSwitchStreamsRequestDeferral_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMediaStreamSourceSwitchStreamsRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMediaStreamSourceSwitchStreamsRequestedEventArgs {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IMediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaStreamSourceSwitchStreamsRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x42202b72_6ea1_4677_981e_350a0da412aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaStreamSourceSwitchStreamsRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct IMediaTrack(::windows_core::IUnknown);
impl IMediaTrack {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrackKind(&self) -> ::windows_core::Result<MediaTrackKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IMediaTrack, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IMediaTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaTrack {}
impl ::core::fmt::Debug for IMediaTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaTrack").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IMediaTrack {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{03e1fafc-c931-491a-b46b-c10ee8c256b7}");
}
unsafe impl ::windows_core::Interface for IMediaTrack {
    type Vtable = IMediaTrack_Vtbl;
}
impl ::core::clone::Clone for IMediaTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMediaTrack {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x03e1fafc_c931_491a_b46b_c10ee8c256b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTrack_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TrackKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaTrackKind) -> ::windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseSourceBuffer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMseSourceBuffer {
    type Vtable = IMseSourceBuffer_Vtbl;
}
impl ::core::clone::Clone for IMseSourceBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMseSourceBuffer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0c1aa3e3_df8d_4079_a3fe_6849184b4e2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseSourceBuffer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub UpdateStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateStarting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdateStarting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdateStarting: usize,
    #[cfg(feature = "Foundation")]
    pub Updated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Updated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateEnded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdateEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdateEnded: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub Aborted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Aborted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAborted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAborted: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MseAppendMode) -> ::windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MseAppendMode) -> ::windows_core::HRESULT,
    pub IsUpdating: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Buffered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Buffered: usize,
    #[cfg(feature = "Foundation")]
    pub TimestampOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TimestampOffset: usize,
    #[cfg(feature = "Foundation")]
    pub SetTimestampOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetTimestampOffset: usize,
    #[cfg(feature = "Foundation")]
    pub AppendWindowStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppendWindowStart: usize,
    #[cfg(feature = "Foundation")]
    pub SetAppendWindowStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAppendWindowStart: usize,
    #[cfg(feature = "Foundation")]
    pub AppendWindowEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AppendWindowEnd: usize,
    #[cfg(feature = "Foundation")]
    pub SetAppendWindowEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAppendWindowEnd: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AppendBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AppendStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendStream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub AppendStreamMaxSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, maxsize: u64) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AppendStreamMaxSize: usize,
    pub Abort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, start: super::super::Foundation::TimeSpan, end: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Remove: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseSourceBufferList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMseSourceBufferList {
    type Vtable = IMseSourceBufferList_Vtbl;
}
impl ::core::clone::Clone for IMseSourceBufferList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMseSourceBufferList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95fae8e7_a8e7_4ebf_8927_145e940ba511);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseSourceBufferList_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SourceBufferAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceBufferAdded: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceBufferAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceBufferAdded: usize,
    #[cfg(feature = "Foundation")]
    pub SourceBufferRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceBufferRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceBufferRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceBufferRemoved: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Buffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Buffers: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseStreamSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMseStreamSource {
    type Vtable = IMseStreamSource_Vtbl;
}
impl ::core::clone::Clone for IMseStreamSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMseStreamSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0b4198d_02f4_4923_88dd_81bc3f360ffa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseStreamSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Opened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Opened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpened: usize,
    #[cfg(feature = "Foundation")]
    pub Ended: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Ended: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnded: usize,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    pub SourceBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ActiveSourceBuffers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReadyState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MseReadyState) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    pub AddSourceBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mimetype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveSourceBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EndOfStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: MseEndOfStreamStatus) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseStreamSource2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMseStreamSource2 {
    type Vtable = IMseStreamSource2_Vtbl;
}
impl ::core::clone::Clone for IMseStreamSource2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMseStreamSource2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x66f57d37_f9e7_418a_9cde_a020e956552b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseStreamSource2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LiveSeekableRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LiveSeekableRange: usize,
    #[cfg(feature = "Foundation")]
    pub SetLiveSeekableRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetLiveSeekableRange: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMseStreamSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IMseStreamSourceStatics {
    type Vtable = IMseStreamSourceStatics_Vtbl;
}
impl ::core::clone::Clone for IMseStreamSourceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IMseStreamSourceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x465c679d_d570_43ce_ba21_0bff5f3fbd0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMseStreamSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsContentTypeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneAnalysisEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneAnalysisEffect {
    type Vtable = ISceneAnalysisEffect_Vtbl;
}
impl ::core::clone::Clone for ISceneAnalysisEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneAnalysisEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc04ba319_ca41_4813_bffd_7b08b0ed2557);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneAnalysisEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HighDynamicRangeAnalyzer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetDesiredAnalysisInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDesiredAnalysisInterval: usize,
    #[cfg(feature = "Foundation")]
    pub DesiredAnalysisInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DesiredAnalysisInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SceneAnalyzed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SceneAnalyzed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSceneAnalyzed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSceneAnalyzed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneAnalysisEffectFrame(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneAnalysisEffectFrame {
    type Vtable = ISceneAnalysisEffectFrame_Vtbl;
}
impl ::core::clone::Clone for ISceneAnalysisEffectFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneAnalysisEffectFrame {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8b10e4c_7fd9_42e1_85eb_6572c297c987);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneAnalysisEffectFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Capture")]
    pub FrameControlValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Capture"))]
    FrameControlValues: usize,
    pub HighDynamicRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneAnalysisEffectFrame2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneAnalysisEffectFrame2 {
    type Vtable = ISceneAnalysisEffectFrame2_Vtbl;
}
impl ::core::clone::Clone for ISceneAnalysisEffectFrame2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneAnalysisEffectFrame2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2d4e29be_061f_47ae_9915_02524b5f9a5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneAnalysisEffectFrame2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AnalysisRecommendation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SceneAnalysisRecommendation) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISceneAnalyzedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISceneAnalyzedEventArgs {
    type Vtable = ISceneAnalyzedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ISceneAnalyzedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISceneAnalyzedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x146b9588_2851_45e4_ad55_44cf8df8db4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISceneAnalyzedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ResultFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct ISingleSelectMediaTrackList(::windows_core::IUnknown);
impl ISingleSelectMediaTrackList {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectedIndexChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<ISingleSelectMediaTrackList, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndexChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSelectedIndexChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSelectedIndexChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SetSelectedIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSelectedIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SelectedIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SelectedIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ISingleSelectMediaTrackList, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for ISingleSelectMediaTrackList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISingleSelectMediaTrackList {}
impl ::core::fmt::Debug for ISingleSelectMediaTrackList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISingleSelectMediaTrackList").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ISingleSelectMediaTrackList {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{77206f1f-c34f-494f-8077-2bad9ff4ecf1}");
}
unsafe impl ::windows_core::Interface for ISingleSelectMediaTrackList {
    type Vtable = ISingleSelectMediaTrackList_Vtbl;
}
impl ::core::clone::Clone for ISingleSelectMediaTrackList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISingleSelectMediaTrackList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77206f1f_c34f_494f_8077_2bad9ff4ecf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISingleSelectMediaTrackList_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SelectedIndexChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectedIndexChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectedIndexChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectedIndexChanged: usize,
    pub SetSelectedIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub SelectedIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISpeechCue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISpeechCue {
    type Vtable = ISpeechCue_Vtbl;
}
impl ::core::clone::Clone for ISpeechCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISpeechCue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaee254dc_1725_4bad_8043_a98499b017a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpeechCue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StartPositionInInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartPositionInInput: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartPositionInInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartPositionInInput: usize,
    #[cfg(feature = "Foundation")]
    pub EndPositionInInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EndPositionInInput: usize,
    #[cfg(feature = "Foundation")]
    pub SetEndPositionInInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEndPositionInInput: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataStreamDescriptor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataStreamDescriptor {
    type Vtable = ITimedMetadataStreamDescriptor_Vtbl;
}
impl ::core::clone::Clone for ITimedMetadataStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedMetadataStreamDescriptor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x133336bf_296a_463e_9ff9_01cd25691408);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataStreamDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataStreamDescriptorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataStreamDescriptorFactory {
    type Vtable = ITimedMetadataStreamDescriptorFactory_Vtbl;
}
impl ::core::clone::Clone for ITimedMetadataStreamDescriptorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedMetadataStreamDescriptorFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc027de30_7362_4ff9_98b1_2dfd0b8d1cae);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataStreamDescriptorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrack(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataTrack {
    type Vtable = ITimedMetadataTrack_Vtbl;
}
impl ::core::clone::Clone for ITimedMetadataTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedMetadataTrack {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9e6aed9e_f67a_49a9_b330_cf03b0e9cf07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrack_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CueEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CueEntered: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCueEntered: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCueEntered: usize,
    #[cfg(feature = "Foundation")]
    pub CueExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CueExited: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCueExited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCueExited: usize,
    #[cfg(feature = "Foundation")]
    pub TrackFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrackFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTrackFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTrackFailed: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Cues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Cues: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ActiveCues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ActiveCues: usize,
    pub TimedMetadataKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataKind) -> ::windows_core::HRESULT,
    pub DispatchType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddCue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveCue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrack2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataTrack2 {
    type Vtable = ITimedMetadataTrack2_Vtbl;
}
impl ::core::clone::Clone for ITimedMetadataTrack2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedMetadataTrack2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x21b4b648_9f9d_40ba_a8f3_1a92753aef0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrack2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_Playback")]
    pub PlaybackItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrackError(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataTrackError {
    type Vtable = ITimedMetadataTrackError_Vtbl;
}
impl ::core::clone::Clone for ITimedMetadataTrackError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedMetadataTrackError {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb3767915_4114_4819_b9d9_dd76089e72f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackError_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedMetadataTrackErrorCode) -> ::windows_core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrackFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataTrackFactory {
    type Vtable = ITimedMetadataTrackFactory_Vtbl;
}
impl ::core::clone::Clone for ITimedMetadataTrackFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedMetadataTrackFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8dd57611_97b3_4e1f_852c_0f482c81ad26);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::std::mem::MaybeUninit<::windows_core::HSTRING>, language: ::std::mem::MaybeUninit<::windows_core::HSTRING>, kind: TimedMetadataKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedMetadataTrackFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedMetadataTrackFailedEventArgs {
    type Vtable = ITimedMetadataTrackFailedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ITimedMetadataTrackFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedMetadataTrackFailedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa57fc9d1_6789_4d4d_b07f_84b4f31acb70);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct ITimedMetadataTrackProvider(::windows_core::IUnknown);
impl ITimedMetadataTrackProvider {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TimedMetadataTracks(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimedMetadataTracks)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(ITimedMetadataTrackProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for ITimedMetadataTrackProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITimedMetadataTrackProvider {}
impl ::core::fmt::Debug for ITimedMetadataTrackProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITimedMetadataTrackProvider").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ITimedMetadataTrackProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{3b7f2024-f74e-4ade-93c5-219da05b6856}");
}
unsafe impl ::windows_core::Interface for ITimedMetadataTrackProvider {
    type Vtable = ITimedMetadataTrackProvider_Vtbl;
}
impl ::core::clone::Clone for ITimedMetadataTrackProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedMetadataTrackProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b7f2024_f74e_4ade_93c5_219da05b6856);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub TimedMetadataTracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TimedMetadataTracks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextBouten(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextBouten {
    type Vtable = ITimedTextBouten_Vtbl;
}
impl ::core::clone::Clone for ITimedTextBouten {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextBouten {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd9062783_5597_5092_820c_8f738e0f774a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextBouten_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextBoutenType) -> ::windows_core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextBoutenType) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextBoutenPosition) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextBoutenPosition) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextCue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextCue {
    type Vtable = ITimedTextCue_Vtbl;
}
impl ::core::clone::Clone for ITimedTextCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextCue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51c79e51_3b86_494d_b359_bb2ea7aca9a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextCue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CueRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCueRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CueStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCueStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Lines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Lines: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextLine(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextLine {
    type Vtable = ITimedTextLine_Vtbl;
}
impl ::core::clone::Clone for ITimedTextLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextLine {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x978d7ce2_7308_4c66_be50_65777289f5df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextLine_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Subformats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Subformats: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextRegion(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextRegion {
    type Vtable = ITimedTextRegion_Vtbl;
}
impl ::core::clone::Clone for ITimedTextRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextRegion {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1ed0881f_8a06_4222_9f59_b21bf40124b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextRegion_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPoint) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextPoint) -> ::windows_core::HRESULT,
    pub Extent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextSize) -> ::windows_core::HRESULT,
    pub SetExtent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextSize) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub Background: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Background: usize,
    #[cfg(feature = "UI")]
    pub SetBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackground: usize,
    pub WritingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWritingMode) -> ::windows_core::HRESULT,
    pub SetWritingMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextWritingMode) -> ::windows_core::HRESULT,
    pub DisplayAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDisplayAlignment) -> ::windows_core::HRESULT,
    pub SetDisplayAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDisplayAlignment) -> ::windows_core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows_core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows_core::HRESULT,
    pub IsOverflowClipped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsOverflowClipped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Padding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextPadding) -> ::windows_core::HRESULT,
    pub SetPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextPadding) -> ::windows_core::HRESULT,
    pub TextWrapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWrapping) -> ::windows_core::HRESULT,
    pub SetTextWrapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextWrapping) -> ::windows_core::HRESULT,
    pub ZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub ScrollMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextScrollMode) -> ::windows_core::HRESULT,
    pub SetScrollMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextScrollMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextRuby(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextRuby {
    type Vtable = ITimedTextRuby_Vtbl;
}
impl ::core::clone::Clone for ITimedTextRuby {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextRuby {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x10335c29_5b3c_5693_9959_d05a0bd24628);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextRuby_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyPosition) -> ::windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextRubyPosition) -> ::windows_core::HRESULT,
    pub Align: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyAlign) -> ::windows_core::HRESULT,
    pub SetAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextRubyAlign) -> ::windows_core::HRESULT,
    pub Reserve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextRubyReserve) -> ::windows_core::HRESULT,
    pub SetReserve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextRubyReserve) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextSource {
    type Vtable = ITimedTextSource_Vtbl;
}
impl ::core::clone::Clone for ITimedTextSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextSource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc4ed9ba6_101f_404d_a949_82f33fcd93b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Resolved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Resolved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResolved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResolved: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSourceResolveResultEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextSourceResolveResultEventArgs {
    type Vtable = ITimedTextSourceResolveResultEventArgs_Vtbl;
}
impl ::core::clone::Clone for ITimedTextSourceResolveResultEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextSourceResolveResultEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48907c9c_dcd8_4c33_9ad3_6cdce7b1c566);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceResolveResultEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Error: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Tracks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tracks: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSourceStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextSourceStatics {
    type Vtable = ITimedTextSourceStatics_Vtbl;
}
impl ::core::clone::Clone for ITimedTextSourceStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextSourceStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e311853_9aba_4ac4_bb98_2fb176c3bfdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStream: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFromUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUri: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, defaultlanguage: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamWithLanguage: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFromUriWithLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, defaultlanguage: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUriWithLanguage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSourceStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextSourceStatics2 {
    type Vtable = ITimedTextSourceStatics2_Vtbl;
}
impl ::core::clone::Clone for ITimedTextSourceStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextSourceStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb66b7602_923e_43fa_9633_587075812db5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamWithIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, indexstream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamWithIndex: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFromUriWithIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, indexuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUriWithIndex: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamWithIndexAndLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, indexstream: *mut ::core::ffi::c_void, defaultlanguage: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamWithIndexAndLanguage: usize,
    #[cfg(feature = "Foundation")]
    pub CreateFromUriWithIndexAndLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, indexuri: *mut ::core::ffi::c_void, defaultlanguage: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateFromUriWithIndexAndLanguage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextStyle(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextStyle {
    type Vtable = ITimedTextStyle_Vtbl;
}
impl ::core::clone::Clone for ITimedTextStyle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextStyle {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1bb2384d_a825_40c2_a7f5_281eaedf3b55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextStyle_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FontFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetFontFamily: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub FontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows_core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows_core::HRESULT,
    pub FontWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextWeight) -> ::windows_core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextWeight) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub Foreground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Foreground: usize,
    #[cfg(feature = "UI")]
    pub SetForeground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetForeground: usize,
    #[cfg(feature = "UI")]
    pub Background: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Background: usize,
    #[cfg(feature = "UI")]
    pub SetBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetBackground: usize,
    pub IsBackgroundAlwaysShown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsBackgroundAlwaysShown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub FlowDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextFlowDirection) -> ::windows_core::HRESULT,
    pub SetFlowDirection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextFlowDirection) -> ::windows_core::HRESULT,
    pub LineAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextLineAlignment) -> ::windows_core::HRESULT,
    pub SetLineAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextLineAlignment) -> ::windows_core::HRESULT,
    #[cfg(feature = "UI")]
    pub OutlineColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    OutlineColor: usize,
    #[cfg(feature = "UI")]
    pub SetOutlineColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetOutlineColor: usize,
    pub OutlineThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows_core::HRESULT,
    pub SetOutlineThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows_core::HRESULT,
    pub OutlineRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextDouble) -> ::windows_core::HRESULT,
    pub SetOutlineRadius: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextDouble) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextStyle2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextStyle2 {
    type Vtable = ITimedTextStyle2_Vtbl;
}
impl ::core::clone::Clone for ITimedTextStyle2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextStyle2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x655f492d_6111_4787_89cc_686fece57e14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextStyle2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimedTextFontStyle) -> ::windows_core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimedTextFontStyle) -> ::windows_core::HRESULT,
    pub IsUnderlineEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsUnderlineEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsLineThroughEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsLineThroughEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub IsOverlineEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsOverlineEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextStyle3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextStyle3 {
    type Vtable = ITimedTextStyle3_Vtbl;
}
impl ::core::clone::Clone for ITimedTextStyle3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextStyle3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf803f93b_3e99_595e_bbb7_78a2fa13c270);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextStyle3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Ruby: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Bouten: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsTextCombined: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsTextCombined: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub FontAngleInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetFontAngleInDegrees: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ITimedTextSubformat(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ITimedTextSubformat {
    type Vtable = ITimedTextSubformat_Vtbl;
}
impl ::core::clone::Clone for ITimedTextSubformat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ITimedTextSubformat {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd713502f_3261_4722_a0c2_b937b2390f14);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSubformat_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub StartIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetStartIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub SubformatStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSubformatStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStabilizationEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoStabilizationEffect {
    type Vtable = IVideoStabilizationEffect_Vtbl;
}
impl ::core::clone::Clone for IVideoStabilizationEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoStabilizationEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0808a650_9698_4e57_877b_bd7cb2ee0f8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStabilizationEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnabledChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnabledChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnabledChanged: usize,
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties"))]
    pub GetRecommendedStreamConfiguration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, controller: *mut ::core::ffi::c_void, desiredproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties")))]
    GetRecommendedStreamConfiguration: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStabilizationEffectEnabledChangedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoStabilizationEffectEnabledChangedEventArgs {
    type Vtable = IVideoStabilizationEffectEnabledChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IVideoStabilizationEffectEnabledChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoStabilizationEffectEnabledChangedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x187eff28_67bb_4713_b900_4168da164529);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStabilizationEffectEnabledChangedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut VideoStabilizationEffectEnabledChangedReason) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStreamDescriptor(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoStreamDescriptor {
    type Vtable = IVideoStreamDescriptor_Vtbl;
}
impl ::core::clone::Clone for IVideoStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoStreamDescriptor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12ee0d55_9c2b_4440_8057_2c7a90f0cbec);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamDescriptor_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub EncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    EncodingProperties: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStreamDescriptor2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoStreamDescriptor2 {
    type Vtable = IVideoStreamDescriptor2_Vtbl;
}
impl ::core::clone::Clone for IVideoStreamDescriptor2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoStreamDescriptor2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8b306e10_453e_4088_832d_c36fa4f94af3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamDescriptor2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoStreamDescriptorFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoStreamDescriptorFactory {
    type Vtable = IVideoStreamDescriptorFactory_Vtbl;
}
impl ::core::clone::Clone for IVideoStreamDescriptorFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoStreamDescriptorFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x494ef6d1_bb75_43d2_9e5e_7b79a3afced4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoStreamDescriptorFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Media_MediaProperties")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingproperties: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTrack(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoTrack {
    type Vtable = IVideoTrack_Vtbl;
}
impl ::core::clone::Clone for IVideoTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoTrack {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x99f3b7f3_e298_4396_bb6a_a51be6a2a20a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTrack_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub OpenFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenFailed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOpenFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOpenFailed: usize,
    #[cfg(feature = "Media_MediaProperties")]
    pub GetEncodingProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_MediaProperties"))]
    GetEncodingProperties: usize,
    #[cfg(feature = "Media_Playback")]
    pub PlaybackItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SupportInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTrackOpenFailedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoTrackOpenFailedEventArgs {
    type Vtable = IVideoTrackOpenFailedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IVideoTrackOpenFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoTrackOpenFailedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7679e231_04f9_4c82_a4ee_8602c8bb4754);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTrackOpenFailedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVideoTrackSupportInfo(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVideoTrackSupportInfo {
    type Vtable = IVideoTrackSupportInfo_Vtbl;
}
impl ::core::clone::Clone for IVideoTrackSupportInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVideoTrackSupportInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4bb534a0_fc5f_450d_8ff0_778d590486de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoTrackSupportInfo_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DecoderStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaDecoderStatus) -> ::windows_core::HRESULT,
    pub MediaSourceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MediaSourceStatus) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct AudioStreamDescriptor(::windows_core::IUnknown);
impl AudioStreamDescriptor {
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLeadingEncoderPadding<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLeadingEncoderPadding)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LeadingEncoderPadding(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows_core::ComInterface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LeadingEncoderPadding)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTrailingEncoderPadding<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<u32>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetTrailingEncoderPadding)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrailingEncoderPadding(&self) -> ::windows_core::Result<super::super::Foundation::IReference<u32>> {
        let this = &::windows_core::ComInterface::cast::<IAudioStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrailingEncoderPadding)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Copy(&self) -> ::windows_core::Result<AudioStreamDescriptor> {
        let this = &::windows_core::ComInterface::cast::<IAudioStreamDescriptor3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Create<P0>(encodingproperties: P0) -> ::windows_core::Result<AudioStreamDescriptor>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::AudioEncodingProperties>,
    {
        Self::IAudioStreamDescriptorFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IAudioStreamDescriptorFactory<R, F: FnOnce(&IAudioStreamDescriptorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<AudioStreamDescriptor, IAudioStreamDescriptorFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for AudioStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioStreamDescriptor {}
impl ::core::fmt::Debug for AudioStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioStreamDescriptor").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioStreamDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.AudioStreamDescriptor;{1e3692e4-4027-4847-a70b-df1d9a2a7b04})");
}
impl ::core::clone::Clone for AudioStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AudioStreamDescriptor {
    type Vtable = IAudioStreamDescriptor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AudioStreamDescriptor {
    const IID: ::windows_core::GUID = <IAudioStreamDescriptor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AudioStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.AudioStreamDescriptor";
}
::windows_core::imp::interface_hierarchy!(AudioStreamDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaStreamDescriptor> for AudioStreamDescriptor {}
impl ::windows_core::CanTryInto<IMediaStreamDescriptor2> for AudioStreamDescriptor {}
unsafe impl ::core::marker::Send for AudioStreamDescriptor {}
unsafe impl ::core::marker::Sync for AudioStreamDescriptor {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct AudioTrack(::windows_core::IUnknown);
impl AudioTrack {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenFailed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<AudioTrack, AudioTrackOpenFailedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOpenFailed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IAudioTrack>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpenFailed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetEncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::AudioEncodingProperties> {
        let this = &::windows_core::ComInterface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEncodingProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Playback\"`*"]
    #[cfg(feature = "Media_Playback")]
    pub fn PlaybackItem(&self) -> ::windows_core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows_core::ComInterface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackItem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SupportInfo(&self) -> ::windows_core::Result<AudioTrackSupportInfo> {
        let this = &::windows_core::ComInterface::cast::<IAudioTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrackKind(&self) -> ::windows_core::Result<MediaTrackKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioTrack {}
impl ::core::fmt::Debug for AudioTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioTrack").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioTrack {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.AudioTrack;{03e1fafc-c931-491a-b46b-c10ee8c256b7})");
}
impl ::core::clone::Clone for AudioTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AudioTrack {
    type Vtable = IMediaTrack_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AudioTrack {
    const IID: ::windows_core::GUID = <IMediaTrack as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AudioTrack {
    const NAME: &'static str = "Windows.Media.Core.AudioTrack";
}
::windows_core::imp::interface_hierarchy!(AudioTrack, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaTrack> for AudioTrack {}
unsafe impl ::core::marker::Send for AudioTrack {}
unsafe impl ::core::marker::Sync for AudioTrack {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct AudioTrackOpenFailedEventArgs(::windows_core::IUnknown);
impl AudioTrackOpenFailedEventArgs {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioTrackOpenFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioTrackOpenFailedEventArgs {}
impl ::core::fmt::Debug for AudioTrackOpenFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioTrackOpenFailedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioTrackOpenFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.AudioTrackOpenFailedEventArgs;{eeddb9b9-bb7c-4112-bf76-9384676f824b})");
}
impl ::core::clone::Clone for AudioTrackOpenFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AudioTrackOpenFailedEventArgs {
    type Vtable = IAudioTrackOpenFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AudioTrackOpenFailedEventArgs {
    const IID: ::windows_core::GUID = <IAudioTrackOpenFailedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AudioTrackOpenFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.AudioTrackOpenFailedEventArgs";
}
::windows_core::imp::interface_hierarchy!(AudioTrackOpenFailedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AudioTrackOpenFailedEventArgs {}
unsafe impl ::core::marker::Sync for AudioTrackOpenFailedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct AudioTrackSupportInfo(::windows_core::IUnknown);
impl AudioTrackSupportInfo {
    pub fn DecoderStatus(&self) -> ::windows_core::Result<MediaDecoderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecoderStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Degradation(&self) -> ::windows_core::Result<AudioDecoderDegradation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Degradation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DegradationReason(&self) -> ::windows_core::Result<AudioDecoderDegradationReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DegradationReason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MediaSourceStatus(&self) -> ::windows_core::Result<MediaSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaSourceStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AudioTrackSupportInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AudioTrackSupportInfo {}
impl ::core::fmt::Debug for AudioTrackSupportInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioTrackSupportInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioTrackSupportInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.AudioTrackSupportInfo;{178beff7-cc39-44a6-b951-4a5653f073fa})");
}
impl ::core::clone::Clone for AudioTrackSupportInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AudioTrackSupportInfo {
    type Vtable = IAudioTrackSupportInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AudioTrackSupportInfo {
    const IID: ::windows_core::GUID = <IAudioTrackSupportInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AudioTrackSupportInfo {
    const NAME: &'static str = "Windows.Media.Core.AudioTrackSupportInfo";
}
::windows_core::imp::interface_hierarchy!(AudioTrackSupportInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AudioTrackSupportInfo {}
unsafe impl ::core::marker::Sync for AudioTrackSupportInfo {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct ChapterCue(::windows_core::IUnknown);
impl ChapterCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ChapterCue, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetTitle(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTitle)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Title(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Title)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ChapterCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ChapterCue {}
impl ::core::fmt::Debug for ChapterCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ChapterCue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ChapterCue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.ChapterCue;{72a98001-d38a-4c0a-8fa6-75cddaf4664c})");
}
impl ::core::clone::Clone for ChapterCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ChapterCue {
    type Vtable = IChapterCue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ChapterCue {
    const IID: ::windows_core::GUID = <IChapterCue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ChapterCue {
    const NAME: &'static str = "Windows.Media.Core.ChapterCue";
}
::windows_core::imp::interface_hierarchy!(ChapterCue, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaCue> for ChapterCue {}
unsafe impl ::core::marker::Send for ChapterCue {}
unsafe impl ::core::marker::Sync for ChapterCue {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct CodecInfo(::windows_core::IUnknown);
impl CodecInfo {
    pub fn Kind(&self) -> ::windows_core::Result<CodecKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Category(&self) -> ::windows_core::Result<CodecCategory> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Category)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Subtypes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subtypes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsTrusted(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTrusted)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CodecInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CodecInfo {}
impl ::core::fmt::Debug for CodecInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CodecInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.CodecInfo;{51e89f85-ea97-499c-86ac-4ce5e73f3a42})");
}
impl ::core::clone::Clone for CodecInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CodecInfo {
    type Vtable = ICodecInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CodecInfo {
    const IID: ::windows_core::GUID = <ICodecInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CodecInfo {
    const NAME: &'static str = "Windows.Media.Core.CodecInfo";
}
::windows_core::imp::interface_hierarchy!(CodecInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CodecInfo {}
unsafe impl ::core::marker::Sync for CodecInfo {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct CodecQuery(::windows_core::IUnknown);
impl CodecQuery {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CodecQuery, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllAsync(&self, kind: CodecKind, category: CodecCategory, subtype: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<CodecInfo>>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FindAllAsync)(::windows_core::Interface::as_raw(this), kind, category, ::core::mem::transmute_copy(subtype), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for CodecQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CodecQuery {}
impl ::core::fmt::Debug for CodecQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecQuery").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CodecQuery {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.CodecQuery;{222a953a-af61-4e04-808a-a4634e2f3ac4})");
}
impl ::core::clone::Clone for CodecQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for CodecQuery {
    type Vtable = ICodecQuery_Vtbl;
}
unsafe impl ::windows_core::ComInterface for CodecQuery {
    const IID: ::windows_core::GUID = <ICodecQuery as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for CodecQuery {
    const NAME: &'static str = "Windows.Media.Core.CodecQuery";
}
::windows_core::imp::interface_hierarchy!(CodecQuery, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for CodecQuery {}
unsafe impl ::core::marker::Sync for CodecQuery {}
#[doc = "*Required features: `\"Media_Core\"`*"]
pub struct CodecSubtypes;
impl CodecSubtypes {
    pub fn VideoFormatDV25() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDV25)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatDV50() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDV50)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatDvc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvc)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatDvh1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvh1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatDvhD() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvhD)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatDvsd() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvsd)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatDvsl() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatDvsl)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatH263() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatH263)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatH264() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatH264)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatH265() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatH265)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatH264ES() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatH264ES)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatHevc() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatHevc)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatHevcES() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatHevcES)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatM4S2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatM4S2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatMjpg() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMjpg)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatMP43() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMP43)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatMP4S() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMP4S)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatMP4V() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMP4V)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatMpeg2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMpeg2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatVP80() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatVP80)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatVP90() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatVP90)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatMpg1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMpg1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatMss1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMss1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatMss2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatMss2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatWmv1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatWmv1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatWmv2() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatWmv2)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatWmv3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatWmv3)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormatWvc1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormatWvc1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn VideoFormat420O() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoFormat420O)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatAac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAac)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatAdts() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAdts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatAlac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAlac)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatAmrNB() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAmrNB)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatAmrWB() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAmrWB)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatAmrWP() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatAmrWP)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatDolbyAC3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDolbyAC3)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatDolbyAC3Spdif() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDolbyAC3Spdif)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatDolbyDDPlus() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDolbyDDPlus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatDrm() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDrm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatDts() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatDts)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatFlac() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatFlac)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatFloat() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatFloat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatMP3() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatMP3)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatMPeg() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatMPeg)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatMsp1() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatMsp1)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatOpus() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatOpus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatPcm() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatPcm)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatWmaSpdif() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatWmaSpdif)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatWMAudioLossless() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatWMAudioLossless)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatWMAudioV8() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatWMAudioV8)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn AudioFormatWMAudioV9() -> ::windows_core::Result<::windows_core::HSTRING> {
        Self::ICodecSubtypesStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AudioFormatWMAudioV9)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICodecSubtypesStatics<R, F: FnOnce(&ICodecSubtypesStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<CodecSubtypes, ICodecSubtypesStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for CodecSubtypes {
    const NAME: &'static str = "Windows.Media.Core.CodecSubtypes";
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct DataCue(::windows_core::IUnknown);
impl DataCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DataCue, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetData)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows_core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Data)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::PropertySet> {
        let this = &::windows_core::ComInterface::cast::<IDataCue2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for DataCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataCue {}
impl ::core::fmt::Debug for DataCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataCue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DataCue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.DataCue;{7c7f676d-1fbc-4e2d-9a87-ee38bd1dc637})");
}
impl ::core::clone::Clone for DataCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DataCue {
    type Vtable = IDataCue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DataCue {
    const IID: ::windows_core::GUID = <IDataCue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DataCue {
    const NAME: &'static str = "Windows.Media.Core.DataCue";
}
::windows_core::imp::interface_hierarchy!(DataCue, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaCue> for DataCue {}
unsafe impl ::core::marker::Send for DataCue {}
unsafe impl ::core::marker::Sync for DataCue {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct FaceDetectedEventArgs(::windows_core::IUnknown);
impl FaceDetectedEventArgs {
    pub fn ResultFrame(&self) -> ::windows_core::Result<FaceDetectionEffectFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResultFrame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for FaceDetectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceDetectedEventArgs {}
impl ::core::fmt::Debug for FaceDetectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FaceDetectedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.FaceDetectedEventArgs;{19918426-c65b-46ba-85f8-13880576c90a})");
}
impl ::core::clone::Clone for FaceDetectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FaceDetectedEventArgs {
    type Vtable = IFaceDetectedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FaceDetectedEventArgs {
    const IID: ::windows_core::GUID = <IFaceDetectedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FaceDetectedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectedEventArgs";
}
::windows_core::imp::interface_hierarchy!(FaceDetectedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for FaceDetectedEventArgs {}
unsafe impl ::core::marker::Sync for FaceDetectedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct FaceDetectionEffect(::windows_core::IUnknown);
impl FaceDetectionEffect {
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredDetectionInterval(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredDetectionInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredDetectionInterval(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredDetectionInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FaceDetected<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<FaceDetectionEffect, FaceDetectedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FaceDetected)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFaceDetected(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveFaceDetected)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.try_into_param()?.abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for FaceDetectionEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceDetectionEffect {}
impl ::core::fmt::Debug for FaceDetectionEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionEffect").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FaceDetectionEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.FaceDetectionEffect;{ae15ebd2-0542-42a9-bc90-f283a29f46c1})");
}
impl ::core::clone::Clone for FaceDetectionEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FaceDetectionEffect {
    type Vtable = IFaceDetectionEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FaceDetectionEffect {
    const IID: ::windows_core::GUID = <IFaceDetectionEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FaceDetectionEffect {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectionEffect";
}
::windows_core::imp::interface_hierarchy!(FaceDetectionEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaExtension> for FaceDetectionEffect {}
unsafe impl ::core::marker::Send for FaceDetectionEffect {}
unsafe impl ::core::marker::Sync for FaceDetectionEffect {}
#[doc = "*Required features: `\"Media_Core\"`, `\"Media_Effects\"`*"]
#[cfg(feature = "Media_Effects")]
#[repr(transparent)]
pub struct FaceDetectionEffectDefinition(::windows_core::IUnknown);
#[cfg(feature = "Media_Effects")]
impl FaceDetectionEffectDefinition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<FaceDetectionEffectDefinition, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SetDetectionMode(&self, value: FaceDetectionMode) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDetectionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DetectionMode(&self) -> ::windows_core::Result<FaceDetectionMode> {
        let this = &::windows_core::ComInterface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetectionMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSynchronousDetectionEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSynchronousDetectionEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SynchronousDetectionEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IFaceDetectionEffectDefinition>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SynchronousDetectionEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::cmp::PartialEq for FaceDetectionEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::cmp::Eq for FaceDetectionEffectDefinition {}
#[cfg(feature = "Media_Effects")]
impl ::core::fmt::Debug for FaceDetectionEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionEffectDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Media_Effects")]
impl ::windows_core::RuntimeType for FaceDetectionEffectDefinition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.FaceDetectionEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
}
#[cfg(feature = "Media_Effects")]
impl ::core::clone::Clone for FaceDetectionEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows_core::Interface for FaceDetectionEffectDefinition {
    type Vtable = super::Effects::IVideoEffectDefinition_Vtbl;
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows_core::ComInterface for FaceDetectionEffectDefinition {
    const IID: ::windows_core::GUID = <super::Effects::IVideoEffectDefinition as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Media_Effects")]
impl ::windows_core::RuntimeName for FaceDetectionEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectionEffectDefinition";
}
#[cfg(feature = "Media_Effects")]
::windows_core::imp::interface_hierarchy!(FaceDetectionEffectDefinition, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Media_Effects")]
impl ::windows_core::CanTryInto<super::Effects::IVideoEffectDefinition> for FaceDetectionEffectDefinition {}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Send for FaceDetectionEffectDefinition {}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Sync for FaceDetectionEffectDefinition {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct FaceDetectionEffectFrame(::windows_core::IUnknown);
impl FaceDetectionEffectFrame {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_FaceAnalysis\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_FaceAnalysis"))]
    pub fn DetectedFaces(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::FaceAnalysis::DetectedFace>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DetectedFaces)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemRelativeTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscontinuous)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for FaceDetectionEffectFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FaceDetectionEffectFrame {}
impl ::core::fmt::Debug for FaceDetectionEffectFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionEffectFrame").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FaceDetectionEffectFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.FaceDetectionEffectFrame;{8ab08993-5dc8-447b-a247-5270bd802ece})");
}
impl ::core::clone::Clone for FaceDetectionEffectFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for FaceDetectionEffectFrame {
    type Vtable = IFaceDetectionEffectFrame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for FaceDetectionEffectFrame {
    const IID: ::windows_core::GUID = <IFaceDetectionEffectFrame as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for FaceDetectionEffectFrame {
    const NAME: &'static str = "Windows.Media.Core.FaceDetectionEffectFrame";
}
::windows_core::imp::interface_hierarchy!(FaceDetectionEffectFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for FaceDetectionEffectFrame {}
impl ::windows_core::CanTryInto<super::IMediaFrame> for FaceDetectionEffectFrame {}
unsafe impl ::core::marker::Send for FaceDetectionEffectFrame {}
unsafe impl ::core::marker::Sync for FaceDetectionEffectFrame {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct HighDynamicRangeControl(::windows_core::IUnknown);
impl HighDynamicRangeControl {
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HighDynamicRangeControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HighDynamicRangeControl {}
impl ::core::fmt::Debug for HighDynamicRangeControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HighDynamicRangeControl").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HighDynamicRangeControl {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.HighDynamicRangeControl;{55f1a7ae-d957-4dc9-9d1c-8553a82a7d99})");
}
impl ::core::clone::Clone for HighDynamicRangeControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for HighDynamicRangeControl {
    type Vtable = IHighDynamicRangeControl_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HighDynamicRangeControl {
    const IID: ::windows_core::GUID = <IHighDynamicRangeControl as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HighDynamicRangeControl {
    const NAME: &'static str = "Windows.Media.Core.HighDynamicRangeControl";
}
::windows_core::imp::interface_hierarchy!(HighDynamicRangeControl, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for HighDynamicRangeControl {}
unsafe impl ::core::marker::Sync for HighDynamicRangeControl {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct HighDynamicRangeOutput(::windows_core::IUnknown);
impl HighDynamicRangeOutput {
    pub fn Certainty(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Certainty)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Devices_Core\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Devices_Core"))]
    pub fn FrameControllers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::Devices::Core::FrameController>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameControllers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for HighDynamicRangeOutput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HighDynamicRangeOutput {}
impl ::core::fmt::Debug for HighDynamicRangeOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HighDynamicRangeOutput").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for HighDynamicRangeOutput {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.HighDynamicRangeOutput;{0f57806b-253b-4119-bb40-3a90e51384f7})");
}
impl ::core::clone::Clone for HighDynamicRangeOutput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for HighDynamicRangeOutput {
    type Vtable = IHighDynamicRangeOutput_Vtbl;
}
unsafe impl ::windows_core::ComInterface for HighDynamicRangeOutput {
    const IID: ::windows_core::GUID = <IHighDynamicRangeOutput as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for HighDynamicRangeOutput {
    const NAME: &'static str = "Windows.Media.Core.HighDynamicRangeOutput";
}
::windows_core::imp::interface_hierarchy!(HighDynamicRangeOutput, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for HighDynamicRangeOutput {}
unsafe impl ::core::marker::Sync for HighDynamicRangeOutput {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct ImageCue(::windows_core::IUnknown);
impl ImageCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ImageCue, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Position(&self) -> ::windows_core::Result<TimedTextPoint> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPosition(&self, value: TimedTextPoint) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Extent(&self) -> ::windows_core::Result<TimedTextSize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Extent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetExtent(&self, value: TimedTextSize) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExtent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetSoftwareBitmap<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSoftwareBitmap)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SoftwareBitmap(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SoftwareBitmap)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ImageCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ImageCue {}
impl ::core::fmt::Debug for ImageCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageCue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ImageCue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.ImageCue;{52828282-367b-440b-9116-3c84570dd270})");
}
impl ::core::clone::Clone for ImageCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ImageCue {
    type Vtable = IImageCue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ImageCue {
    const IID: ::windows_core::GUID = <IImageCue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ImageCue {
    const NAME: &'static str = "Windows.Media.Core.ImageCue";
}
::windows_core::imp::interface_hierarchy!(ImageCue, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaCue> for ImageCue {}
unsafe impl ::core::marker::Send for ImageCue {}
unsafe impl ::core::marker::Sync for ImageCue {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct InitializeMediaStreamSourceRequestedEventArgs(::windows_core::IUnknown);
impl InitializeMediaStreamSourceRequestedEventArgs {
    pub fn Source(&self) -> ::windows_core::Result<MediaStreamSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn RandomAccessStream(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RandomAccessStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InitializeMediaStreamSourceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InitializeMediaStreamSourceRequestedEventArgs {}
impl ::core::fmt::Debug for InitializeMediaStreamSourceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InitializeMediaStreamSourceRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for InitializeMediaStreamSourceRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.InitializeMediaStreamSourceRequestedEventArgs;{25bc45e1-9b08-4c2e-a855-4542f1a75deb})");
}
impl ::core::clone::Clone for InitializeMediaStreamSourceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for InitializeMediaStreamSourceRequestedEventArgs {
    type Vtable = IInitializeMediaStreamSourceRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for InitializeMediaStreamSourceRequestedEventArgs {
    const IID: ::windows_core::GUID = <IInitializeMediaStreamSourceRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for InitializeMediaStreamSourceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.InitializeMediaStreamSourceRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(InitializeMediaStreamSourceRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for InitializeMediaStreamSourceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for InitializeMediaStreamSourceRequestedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
pub struct LowLightFusion;
impl LowLightFusion {
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn SupportedBitmapPixelFormats() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<super::super::Graphics::Imaging::BitmapPixelFormat>> {
        Self::ILowLightFusionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportedBitmapPixelFormats)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MaxSupportedFrameCount() -> ::windows_core::Result<i32> {
        Self::ILowLightFusionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSupportedFrameCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Graphics_Imaging"))]
    pub fn FuseAsync<P0>(frameset: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<LowLightFusionResult, f64>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Graphics::Imaging::SoftwareBitmap>>,
    {
        Self::ILowLightFusionStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FuseAsync)(::windows_core::Interface::as_raw(this), frameset.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILowLightFusionStatics<R, F: FnOnce(&ILowLightFusionStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LowLightFusion, ILowLightFusionStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for LowLightFusion {
    const NAME: &'static str = "Windows.Media.Core.LowLightFusion";
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct LowLightFusionResult(::windows_core::IUnknown);
impl LowLightFusionResult {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn Frame(&self) -> ::windows_core::Result<super::super::Graphics::Imaging::SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for LowLightFusionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LowLightFusionResult {}
impl ::core::fmt::Debug for LowLightFusionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LowLightFusionResult").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LowLightFusionResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.LowLightFusionResult;{78edbe35-27a0-42e0-9cd3-738d2089de9c})");
}
impl ::core::clone::Clone for LowLightFusionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LowLightFusionResult {
    type Vtable = ILowLightFusionResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LowLightFusionResult {
    const IID: ::windows_core::GUID = <ILowLightFusionResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LowLightFusionResult {
    const NAME: &'static str = "Windows.Media.Core.LowLightFusionResult";
}
::windows_core::imp::interface_hierarchy!(LowLightFusionResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for LowLightFusionResult {}
unsafe impl ::core::marker::Send for LowLightFusionResult {}
unsafe impl ::core::marker::Sync for LowLightFusionResult {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaBinder(::windows_core::IUnknown);
impl MediaBinder {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaBinder, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Binding<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaBinder, MediaBindingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Binding)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBinding(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBinding)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Token(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Token)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetToken(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetToken)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Source(&self) -> ::windows_core::Result<MediaSource> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Source)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaBinder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBinder {}
impl ::core::fmt::Debug for MediaBinder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBinder").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaBinder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaBinder;{2b7e40aa-de07-424f-83f1-f1de46c4fa2e})");
}
impl ::core::clone::Clone for MediaBinder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaBinder {
    type Vtable = IMediaBinder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaBinder {
    const IID: ::windows_core::GUID = <IMediaBinder as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaBinder {
    const NAME: &'static str = "Windows.Media.Core.MediaBinder";
}
::windows_core::imp::interface_hierarchy!(MediaBinder, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaBinder {}
unsafe impl ::core::marker::Sync for MediaBinder {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaBindingEventArgs(::windows_core::IUnknown);
impl MediaBindingEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Canceled<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaBindingEventArgs, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Canceled)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCanceled(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCanceled)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn MediaBinder(&self) -> ::windows_core::Result<MediaBinder> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaBinder)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<P0>(&self, uri: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStream<P0>(&self, stream: P0, contenttype: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStream)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), ::core::mem::transmute_copy(contenttype)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStreamReference<P0>(&self, stream: P0, contenttype: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStreamReference)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), ::core::mem::transmute_copy(contenttype)).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub fn SetAdaptiveMediaSource<P0>(&self, mediasource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Streaming::Adaptive::AdaptiveMediaSource>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaBindingEventArgs2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetAdaptiveMediaSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SetStorageFile<P0>(&self, file: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaBindingEventArgs2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStorageFile)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub fn SetDownloadOperation<P0>(&self, downloadoperation: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Networking::BackgroundTransfer::DownloadOperation>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaBindingEventArgs3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDownloadOperation)(::windows_core::Interface::as_raw(this), downloadoperation.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for MediaBindingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaBindingEventArgs {}
impl ::core::fmt::Debug for MediaBindingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaBindingEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaBindingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaBindingEventArgs;{b61cb25a-1b6d-4630-a86d-2f0837f712e5})");
}
impl ::core::clone::Clone for MediaBindingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaBindingEventArgs {
    type Vtable = IMediaBindingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaBindingEventArgs {
    const IID: ::windows_core::GUID = <IMediaBindingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaBindingEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaBindingEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaBindingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaBindingEventArgs {}
unsafe impl ::core::marker::Sync for MediaBindingEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaCueEventArgs(::windows_core::IUnknown);
impl MediaCueEventArgs {
    pub fn Cue(&self) -> ::windows_core::Result<IMediaCue> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cue)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaCueEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaCueEventArgs {}
impl ::core::fmt::Debug for MediaCueEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaCueEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaCueEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaCueEventArgs;{d12f47f7-5fa4-4e68-9fe5-32160dcee57e})");
}
impl ::core::clone::Clone for MediaCueEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaCueEventArgs {
    type Vtable = IMediaCueEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaCueEventArgs {
    const IID: ::windows_core::GUID = <IMediaCueEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaCueEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaCueEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaCueEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaCueEventArgs {}
unsafe impl ::core::marker::Sync for MediaCueEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaSource(::windows_core::IUnknown);
impl MediaSource {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenOperationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceOpenOperationCompletedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenOperationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOpenOperationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpenOperationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CustomProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CustomProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsOpen(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOpen)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExternalTimedTextSources(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IObservableVector<TimedTextSource>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExternalTimedTextSources)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExternalTimedMetadataTracks(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IObservableVector<TimedMetadataTrack>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExternalTimedMetadataTracks)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StateChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaSource, MediaSourceStateChangedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaSource3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StateChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStateChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaSource3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStateChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn State(&self) -> ::windows_core::Result<MediaSourceState> {
        let this = &::windows_core::ComInterface::cast::<IMediaSource3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).State)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaSource3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub fn AdaptiveMediaSource(&self) -> ::windows_core::Result<super::Streaming::Adaptive::AdaptiveMediaSource> {
        let this = &::windows_core::ComInterface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AdaptiveMediaSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MediaStreamSource(&self) -> ::windows_core::Result<MediaStreamSource> {
        let this = &::windows_core::ComInterface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaStreamSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MseStreamSource(&self) -> ::windows_core::Result<MseStreamSource> {
        let this = &::windows_core::ComInterface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MseStreamSource)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::Foundation::Uri> {
        let this = &::windows_core::ComInterface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub fn DownloadOperation(&self) -> ::windows_core::Result<super::super::Networking::BackgroundTransfer::DownloadOperation> {
        let this = &::windows_core::ComInterface::cast::<IMediaSource5>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DownloadOperation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Streaming_Adaptive\"`*"]
    #[cfg(feature = "Media_Streaming_Adaptive")]
    pub fn CreateFromAdaptiveMediaSource<P0>(mediasource: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<super::Streaming::Adaptive::AdaptiveMediaSource>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromAdaptiveMediaSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromMediaStreamSource<P0>(mediasource: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<MediaStreamSource>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMediaStreamSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromMseStreamSource<P0>(mediasource: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<MseStreamSource>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMseStreamSource)(::windows_core::Interface::as_raw(this), mediasource.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromIMediaSource<P0>(mediasource: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::TryIntoParam<IMediaSource>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromIMediaSource)(::windows_core::Interface::as_raw(this), mediasource.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn CreateFromStorageFile<P0>(file: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::IStorageFile>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStorageFile)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStream<P0>(stream: P0, contenttype: &::windows_core::HSTRING) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStream)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), ::core::mem::transmute_copy(contenttype), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamReference<P0>(stream: P0, contenttype: &::windows_core::HSTRING) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamReference)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), ::core::mem::transmute_copy(contenttype), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUri<P0>(uri: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromMediaBinder<P0>(binder: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<MediaBinder>,
    {
        Self::IMediaSourceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMediaBinder)(::windows_core::Interface::as_raw(this), binder.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Media_Capture_Frames\"`*"]
    #[cfg(feature = "Media_Capture_Frames")]
    pub fn CreateFromMediaFrameSource<P0>(framesource: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<super::Capture::Frames::MediaFrameSource>,
    {
        Self::IMediaSourceStatics3(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromMediaFrameSource)(::windows_core::Interface::as_raw(this), framesource.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Networking_BackgroundTransfer\"`*"]
    #[cfg(feature = "Networking_BackgroundTransfer")]
    pub fn CreateFromDownloadOperation<P0>(downloadoperation: P0) -> ::windows_core::Result<MediaSource>
    where
        P0: ::windows_core::IntoParam<super::super::Networking::BackgroundTransfer::DownloadOperation>,
    {
        Self::IMediaSourceStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDownloadOperation)(::windows_core::Interface::as_raw(this), downloadoperation.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaSourceStatics<R, F: FnOnce(&IMediaSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaSource, IMediaSourceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaSourceStatics2<R, F: FnOnce(&IMediaSourceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaSource, IMediaSourceStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaSourceStatics3<R, F: FnOnce(&IMediaSourceStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaSource, IMediaSourceStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaSourceStatics4<R, F: FnOnce(&IMediaSourceStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaSource, IMediaSourceStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MediaSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSource {}
impl ::core::fmt::Debug for MediaSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSource;{2eb61048-655f-4c37-b813-b4e45dfa0abe})");
}
impl ::core::clone::Clone for MediaSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaSource {
    type Vtable = IMediaSource2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaSource {
    const IID: ::windows_core::GUID = <IMediaSource2 as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaSource {
    const NAME: &'static str = "Windows.Media.Core.MediaSource";
}
::windows_core::imp::interface_hierarchy!(MediaSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for MediaSource {}
#[cfg(feature = "Media_Playback")]
impl ::windows_core::CanTryInto<super::Playback::IMediaPlaybackSource> for MediaSource {}
unsafe impl ::core::marker::Send for MediaSource {}
unsafe impl ::core::marker::Sync for MediaSource {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaSourceAppServiceConnection(::windows_core::IUnknown);
impl MediaSourceAppServiceConnection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InitializeMediaStreamSourceRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaSourceAppServiceConnection, InitializeMediaStreamSourceRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InitializeMediaStreamSourceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveInitializeMediaStreamSourceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveInitializeMediaStreamSourceRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_AppService\"`*"]
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn Create<P0>(appserviceconnection: P0) -> ::windows_core::Result<MediaSourceAppServiceConnection>
    where
        P0: ::windows_core::IntoParam<super::super::ApplicationModel::AppService::AppServiceConnection>,
    {
        Self::IMediaSourceAppServiceConnectionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), appserviceconnection.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaSourceAppServiceConnectionFactory<R, F: FnOnce(&IMediaSourceAppServiceConnectionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaSourceAppServiceConnection, IMediaSourceAppServiceConnectionFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MediaSourceAppServiceConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceAppServiceConnection {}
impl ::core::fmt::Debug for MediaSourceAppServiceConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceAppServiceConnection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaSourceAppServiceConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSourceAppServiceConnection;{61e1ea97-1916-4810-b7f4-b642be829596})");
}
impl ::core::clone::Clone for MediaSourceAppServiceConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaSourceAppServiceConnection {
    type Vtable = IMediaSourceAppServiceConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaSourceAppServiceConnection {
    const IID: ::windows_core::GUID = <IMediaSourceAppServiceConnection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceAppServiceConnection {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceAppServiceConnection";
}
::windows_core::imp::interface_hierarchy!(MediaSourceAppServiceConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaSourceError(::windows_core::IUnknown);
impl MediaSourceError {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaSourceError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceError {}
impl ::core::fmt::Debug for MediaSourceError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceError").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaSourceError {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSourceError;{5c0a8965-37c5-4e9d-8d21-1cdee90cecc6})");
}
impl ::core::clone::Clone for MediaSourceError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaSourceError {
    type Vtable = IMediaSourceError_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaSourceError {
    const IID: ::windows_core::GUID = <IMediaSourceError as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceError {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceError";
}
::windows_core::imp::interface_hierarchy!(MediaSourceError, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaSourceError {}
unsafe impl ::core::marker::Sync for MediaSourceError {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaSourceOpenOperationCompletedEventArgs(::windows_core::IUnknown);
impl MediaSourceOpenOperationCompletedEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<MediaSourceError> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaSourceOpenOperationCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceOpenOperationCompletedEventArgs {}
impl ::core::fmt::Debug for MediaSourceOpenOperationCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceOpenOperationCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaSourceOpenOperationCompletedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSourceOpenOperationCompletedEventArgs;{fc682ceb-e281-477c-a8e0-1acd654114c8})");
}
impl ::core::clone::Clone for MediaSourceOpenOperationCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaSourceOpenOperationCompletedEventArgs {
    type Vtable = IMediaSourceOpenOperationCompletedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaSourceOpenOperationCompletedEventArgs {
    const IID: ::windows_core::GUID = <IMediaSourceOpenOperationCompletedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceOpenOperationCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceOpenOperationCompletedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaSourceOpenOperationCompletedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaSourceOpenOperationCompletedEventArgs {}
unsafe impl ::core::marker::Sync for MediaSourceOpenOperationCompletedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaSourceStateChangedEventArgs(::windows_core::IUnknown);
impl MediaSourceStateChangedEventArgs {
    pub fn OldState(&self) -> ::windows_core::Result<MediaSourceState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NewState(&self) -> ::windows_core::Result<MediaSourceState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaSourceStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaSourceStateChangedEventArgs {}
impl ::core::fmt::Debug for MediaSourceStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaSourceStateChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaSourceStateChangedEventArgs;{0a30af82-9071-4bac-bc39-ca2a93b717a9})");
}
impl ::core::clone::Clone for MediaSourceStateChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaSourceStateChangedEventArgs {
    type Vtable = IMediaSourceStateChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaSourceStateChangedEventArgs {
    const IID: ::windows_core::GUID = <IMediaSourceStateChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaSourceStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceStateChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaSourceStateChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaSourceStateChangedEventArgs {}
unsafe impl ::core::marker::Sync for MediaSourceStateChangedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSample(::windows_core::IUnknown);
impl MediaStreamSample {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Processed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSample, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Processed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveProcessed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveProcessed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Buffer(&self) -> ::windows_core::Result<super::super::Storage::Streams::Buffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Buffer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Timestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<MediaStreamSamplePropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Protection(&self) -> ::windows_core::Result<MediaStreamSampleProtectionProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Protection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDecodeTimestamp(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDecodeTimestamp)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DecodeTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecodeTimestamp)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetKeyFrame(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeyFrame)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyFrame(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).KeyFrame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Discontinuous(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Discontinuous)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Direct3D11Surface(&self) -> ::windows_core::Result<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamSample2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Direct3D11Surface)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateFromBuffer<P0>(buffer: P0, timestamp: super::super::Foundation::TimeSpan) -> ::windows_core::Result<MediaStreamSample>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::IMediaStreamSampleStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromBuffer)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), timestamp, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateFromStreamAsync<P0>(stream: P0, count: u32, timestamp: super::super::Foundation::TimeSpan) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<MediaStreamSample>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IInputStream>,
    {
        Self::IMediaStreamSampleStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamAsync)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), count, timestamp, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateFromDirect3D11Surface<P0>(surface: P0, timestamp: super::super::Foundation::TimeSpan) -> ::windows_core::Result<MediaStreamSample>
    where
        P0: ::windows_core::TryIntoParam<super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>,
    {
        Self::IMediaStreamSampleStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDirect3D11Surface)(::windows_core::Interface::as_raw(this), surface.try_into_param()?.abi(), timestamp, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaStreamSampleStatics<R, F: FnOnce(&IMediaStreamSampleStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaStreamSample, IMediaStreamSampleStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IMediaStreamSampleStatics2<R, F: FnOnce(&IMediaStreamSampleStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaStreamSample, IMediaStreamSampleStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MediaStreamSample {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSample {}
impl ::core::fmt::Debug for MediaStreamSample {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSample").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSample {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSample;{5c8db627-4b80-4361-9837-6cb7481ad9d6})");
}
impl ::core::clone::Clone for MediaStreamSample {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSample {
    type Vtable = IMediaStreamSample_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSample {
    const IID: ::windows_core::GUID = <IMediaStreamSample as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSample {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSample";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSample, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSample {}
unsafe impl ::core::marker::Sync for MediaStreamSample {}
#[doc = "*Required features: `\"Media_Core\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct MediaStreamSamplePropertySet(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl MediaStreamSamplePropertySet {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), key, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: ::windows_core::GUID) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), key, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::GUID, ::windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<P0>(&self, key: ::windows_core::GUID, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), key, value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: ::windows_core::GUID) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), key).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for MediaStreamSamplePropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for MediaStreamSamplePropertySet {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for MediaStreamSamplePropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSamplePropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for MediaStreamSamplePropertySet {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSamplePropertySet;pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1};g16;cinterface(IInspectable)))");
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for MediaStreamSamplePropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for MediaStreamSamplePropertySet {
    type Vtable = super::super::Foundation::Collections::IMap_Vtbl<::windows_core::GUID, ::windows_core::IInspectable>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for MediaStreamSamplePropertySet {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IMap<::windows_core::GUID, ::windows_core::IInspectable> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for MediaStreamSamplePropertySet {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSamplePropertySet";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for MediaStreamSamplePropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &MediaStreamSamplePropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(MediaStreamSamplePropertySet, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::GUID, ::windows_core::IInspectable>>> for MediaStreamSamplePropertySet {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IMap<::windows_core::GUID, ::windows_core::IInspectable>> for MediaStreamSamplePropertySet {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for MediaStreamSamplePropertySet {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for MediaStreamSamplePropertySet {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSampleProtectionProperties(::windows_core::IUnknown);
impl MediaStreamSampleProtectionProperties {
    pub fn SetKeyIdentifier(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetKeyIdentifier)(::windows_core::Interface::as_raw(this), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn GetKeyIdentifier(&self, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetKeyIdentifier)(::windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn SetInitializationVector(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInitializationVector)(::windows_core::Interface::as_raw(this), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn GetInitializationVector(&self, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetInitializationVector)(::windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn SetSubSampleMapping(&self, value: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubSampleMapping)(::windows_core::Interface::as_raw(this), value.len() as u32, value.as_ptr()).ok() }
    }
    pub fn GetSubSampleMapping(&self, value: &mut ::windows_core::Array<u8>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).GetSubSampleMapping)(::windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSampleProtectionProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSampleProtectionProperties {}
impl ::core::fmt::Debug for MediaStreamSampleProtectionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSampleProtectionProperties").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSampleProtectionProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSampleProtectionProperties;{4eb88292-ecdf-493e-841d-dd4add7caca2})");
}
impl ::core::clone::Clone for MediaStreamSampleProtectionProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSampleProtectionProperties {
    type Vtable = IMediaStreamSampleProtectionProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSampleProtectionProperties {
    const IID: ::windows_core::GUID = <IMediaStreamSampleProtectionProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSampleProtectionProperties {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSampleProtectionProperties";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSampleProtectionProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSampleProtectionProperties {}
unsafe impl ::core::marker::Sync for MediaStreamSampleProtectionProperties {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSource(::windows_core::IUnknown);
impl MediaStreamSource {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceClosedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Starting<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Starting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStarting(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStarting)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Paused<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSource, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Paused)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePaused(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePaused)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SampleRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SampleRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSampleRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSampleRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SwitchStreamsRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSwitchStreamsRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SwitchStreamsRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSwitchStreamsRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSwitchStreamsRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn NotifyError(&self, errorstatus: MediaStreamSourceErrorStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).NotifyError)(::windows_core::Interface::as_raw(this), errorstatus).ok() }
    }
    pub fn AddStreamDescriptor<P0>(&self, descriptor: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IMediaStreamDescriptor>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddStreamDescriptor)(::windows_core::Interface::as_raw(this), descriptor.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Protection\"`*"]
    #[cfg(feature = "Media_Protection")]
    pub fn SetMediaProtectionManager<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Protection::MediaProtectionManager>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMediaProtectionManager)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Protection\"`*"]
    #[cfg(feature = "Media_Protection")]
    pub fn MediaProtectionManager(&self) -> ::windows_core::Result<super::Protection::MediaProtectionManager> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaProtectionManager)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCanSeek(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCanSeek)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanSeek(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanSeek)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBufferTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBufferTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BufferTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BufferTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBufferedRange(&self, startoffset: super::super::Foundation::TimeSpan, endoffset: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBufferedRange)(::windows_core::Interface::as_raw(this), startoffset, endoffset).ok() }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows_core::Result<super::super::Storage::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MusicProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows_core::Result<super::super::Storage::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).VideoProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetThumbnail<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetThumbnail)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Thumbnail)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddProtectionKey<P0>(&self, streamdescriptor: P0, keyidentifier: &[u8], licensedata: &[u8]) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IMediaStreamDescriptor>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddProtectionKey)(::windows_core::Interface::as_raw(this), streamdescriptor.try_into_param()?.abi(), keyidentifier.len() as u32, keyidentifier.as_ptr(), licensedata.len() as u32, licensedata.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SampleRendered<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MediaStreamSource, MediaStreamSourceSampleRenderedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SampleRendered)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSampleRendered(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSampleRendered)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetMaxSupportedPlaybackRate<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<f64>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamSource3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetMaxSupportedPlaybackRate)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxSupportedPlaybackRate(&self) -> ::windows_core::Result<super::super::Foundation::IReference<f64>> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamSource3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaxSupportedPlaybackRate)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsLive(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamSource4>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLive)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLive(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamSource4>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsLive)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateFromDescriptor<P0>(descriptor: P0) -> ::windows_core::Result<MediaStreamSource>
    where
        P0: ::windows_core::TryIntoParam<IMediaStreamDescriptor>,
    {
        Self::IMediaStreamSourceFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDescriptor)(::windows_core::Interface::as_raw(this), descriptor.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateFromDescriptors<P0, P1>(descriptor: P0, descriptor2: P1) -> ::windows_core::Result<MediaStreamSource>
    where
        P0: ::windows_core::TryIntoParam<IMediaStreamDescriptor>,
        P1: ::windows_core::TryIntoParam<IMediaStreamDescriptor>,
    {
        Self::IMediaStreamSourceFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromDescriptors)(::windows_core::Interface::as_raw(this), descriptor.try_into_param()?.abi(), descriptor2.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMediaStreamSourceFactory<R, F: FnOnce(&IMediaStreamSourceFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MediaStreamSource, IMediaStreamSourceFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MediaStreamSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSource {}
impl ::core::fmt::Debug for MediaStreamSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSource;{3712d543-45eb-4138-aa62-c01e26f3843f})");
}
impl ::core::clone::Clone for MediaStreamSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSource {
    type Vtable = IMediaStreamSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSource {
    const IID: ::windows_core::GUID = <IMediaStreamSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSource {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSource";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaSource> for MediaStreamSource {}
unsafe impl ::core::marker::Send for MediaStreamSource {}
unsafe impl ::core::marker::Sync for MediaStreamSource {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceClosedEventArgs(::windows_core::IUnknown);
impl MediaStreamSourceClosedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<MediaStreamSourceClosedRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceClosedEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceClosedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceClosedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceClosedEventArgs;{cd8c7eb2-4816-4e24-88f0-491ef7386406})");
}
impl ::core::clone::Clone for MediaStreamSourceClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceClosedEventArgs {
    type Vtable = IMediaStreamSourceClosedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSourceClosedEventArgs {
    const IID: ::windows_core::GUID = <IMediaStreamSourceClosedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceClosedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceClosedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSourceClosedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSourceClosedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceClosedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceClosedRequest(::windows_core::IUnknown);
impl MediaStreamSourceClosedRequest {
    pub fn Reason(&self) -> ::windows_core::Result<MediaStreamSourceClosedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceClosedRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceClosedRequest {}
impl ::core::fmt::Debug for MediaStreamSourceClosedRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceClosedRequest").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceClosedRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceClosedRequest;{907c00e9-18a3-4951-887a-2c1eebd5c69e})");
}
impl ::core::clone::Clone for MediaStreamSourceClosedRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceClosedRequest {
    type Vtable = IMediaStreamSourceClosedRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSourceClosedRequest {
    const IID: ::windows_core::GUID = <IMediaStreamSourceClosedRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceClosedRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceClosedRequest";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSourceClosedRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSourceClosedRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceClosedRequest {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceSampleRenderedEventArgs(::windows_core::IUnknown);
impl MediaStreamSourceSampleRenderedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SampleLag(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SampleLag)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSampleRenderedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSampleRenderedEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceSampleRenderedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSampleRenderedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSampleRenderedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSampleRenderedEventArgs;{9d697b05-d4f2-4c7a-9dfe-8d6cd0b3ee84})");
}
impl ::core::clone::Clone for MediaStreamSourceSampleRenderedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSampleRenderedEventArgs {
    type Vtable = IMediaStreamSourceSampleRenderedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSourceSampleRenderedEventArgs {
    const IID: ::windows_core::GUID = <IMediaStreamSourceSampleRenderedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSampleRenderedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRenderedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSampleRenderedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRenderedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRenderedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequest(::windows_core::IUnknown);
impl MediaStreamSourceSampleRequest {
    pub fn StreamDescriptor(&self) -> ::windows_core::Result<IMediaStreamDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StreamDescriptor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<MediaStreamSourceSampleRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSample<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MediaStreamSample>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSample)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Sample(&self) -> ::windows_core::Result<MediaStreamSample> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Sample)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReportSampleProgress(&self, progress: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportSampleProgress)(::windows_core::Interface::as_raw(this), progress).ok() }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSampleRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSampleRequest {}
impl ::core::fmt::Debug for MediaStreamSourceSampleRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSampleRequest").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSampleRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSampleRequest;{4db341a9-3501-4d9b-83f9-8f235c822532})");
}
impl ::core::clone::Clone for MediaStreamSourceSampleRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSampleRequest {
    type Vtable = IMediaStreamSourceSampleRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSourceSampleRequest {
    const IID: ::windows_core::GUID = <IMediaStreamSourceSampleRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSampleRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRequest";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSampleRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRequest {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequestDeferral(::windows_core::IUnknown);
impl MediaStreamSourceSampleRequestDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSampleRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSampleRequestDeferral {}
impl ::core::fmt::Debug for MediaStreamSourceSampleRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSampleRequestDeferral").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSampleRequestDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSampleRequestDeferral;{7895cc02-f982-43c8-9d16-c62d999319be})");
}
impl ::core::clone::Clone for MediaStreamSourceSampleRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSampleRequestDeferral {
    type Vtable = IMediaStreamSourceSampleRequestDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSourceSampleRequestDeferral {
    const IID: ::windows_core::GUID = <IMediaStreamSourceSampleRequestDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSampleRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRequestDeferral";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSampleRequestDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRequestDeferral {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRequestDeferral {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceSampleRequestedEventArgs(::windows_core::IUnknown);
impl MediaStreamSourceSampleRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<MediaStreamSourceSampleRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSampleRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSampleRequestedEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceSampleRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSampleRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSampleRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSampleRequestedEventArgs;{10f9bb9e-71c5-492f-847f-0da1f35e81f8})");
}
impl ::core::clone::Clone for MediaStreamSourceSampleRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSampleRequestedEventArgs {
    type Vtable = IMediaStreamSourceSampleRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSourceSampleRequestedEventArgs {
    const IID: ::windows_core::GUID = <IMediaStreamSourceSampleRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSampleRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSampleRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSampleRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSourceSampleRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSampleRequestedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceStartingEventArgs(::windows_core::IUnknown);
impl MediaStreamSourceStartingEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<MediaStreamSourceStartingRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceStartingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceStartingEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceStartingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceStartingEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceStartingEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceStartingEventArgs;{f41468f2-c274-4940-a5bb-28a572452fa7})");
}
impl ::core::clone::Clone for MediaStreamSourceStartingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceStartingEventArgs {
    type Vtable = IMediaStreamSourceStartingEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSourceStartingEventArgs {
    const IID: ::windows_core::GUID = <IMediaStreamSourceStartingEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceStartingEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceStartingEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSourceStartingEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSourceStartingEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceStartingEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceStartingRequest(::windows_core::IUnknown);
impl MediaStreamSourceStartingRequest {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPosition(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPosition)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<MediaStreamSourceStartingRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetActualStartPosition(&self, position: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetActualStartPosition)(::windows_core::Interface::as_raw(this), position).ok() }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceStartingRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceStartingRequest {}
impl ::core::fmt::Debug for MediaStreamSourceStartingRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceStartingRequest").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceStartingRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceStartingRequest;{2a9093e4-35c4-4b1b-a791-0d99db56dd1d})");
}
impl ::core::clone::Clone for MediaStreamSourceStartingRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceStartingRequest {
    type Vtable = IMediaStreamSourceStartingRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSourceStartingRequest {
    const IID: ::windows_core::GUID = <IMediaStreamSourceStartingRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceStartingRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceStartingRequest";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSourceStartingRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSourceStartingRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceStartingRequest {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceStartingRequestDeferral(::windows_core::IUnknown);
impl MediaStreamSourceStartingRequestDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceStartingRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceStartingRequestDeferral {}
impl ::core::fmt::Debug for MediaStreamSourceStartingRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceStartingRequestDeferral").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceStartingRequestDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceStartingRequestDeferral;{3f1356a5-6340-4dc4-9910-068ed9f598f8})");
}
impl ::core::clone::Clone for MediaStreamSourceStartingRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceStartingRequestDeferral {
    type Vtable = IMediaStreamSourceStartingRequestDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSourceStartingRequestDeferral {
    const IID: ::windows_core::GUID = <IMediaStreamSourceStartingRequestDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceStartingRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceStartingRequestDeferral";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSourceStartingRequestDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSourceStartingRequestDeferral {}
unsafe impl ::core::marker::Sync for MediaStreamSourceStartingRequestDeferral {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequest(::windows_core::IUnknown);
impl MediaStreamSourceSwitchStreamsRequest {
    pub fn OldStreamDescriptor(&self) -> ::windows_core::Result<IMediaStreamDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OldStreamDescriptor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn NewStreamDescriptor(&self) -> ::windows_core::Result<IMediaStreamDescriptor> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).NewStreamDescriptor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<MediaStreamSourceSwitchStreamsRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSwitchStreamsRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSwitchStreamsRequest {}
impl ::core::fmt::Debug for MediaStreamSourceSwitchStreamsRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSwitchStreamsRequest").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSwitchStreamsRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSwitchStreamsRequest;{41b8808e-38a9-4ec3-9ba0-b69b85501e90})");
}
impl ::core::clone::Clone for MediaStreamSourceSwitchStreamsRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSwitchStreamsRequest {
    type Vtable = IMediaStreamSourceSwitchStreamsRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSourceSwitchStreamsRequest {
    const IID: ::windows_core::GUID = <IMediaStreamSourceSwitchStreamsRequest as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSwitchStreamsRequest {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSwitchStreamsRequest";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSwitchStreamsRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSourceSwitchStreamsRequest {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSwitchStreamsRequest {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequestDeferral(::windows_core::IUnknown);
impl MediaStreamSourceSwitchStreamsRequestDeferral {
    pub fn Complete(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Complete)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSwitchStreamsRequestDeferral {}
impl ::core::fmt::Debug for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSwitchStreamsRequestDeferral").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSwitchStreamsRequestDeferral {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestDeferral;{bee3d835-a505-4f9a-b943-2b8cb1b4bbd9})");
}
impl ::core::clone::Clone for MediaStreamSourceSwitchStreamsRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSwitchStreamsRequestDeferral {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestDeferral_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSourceSwitchStreamsRequestDeferral {
    const IID: ::windows_core::GUID = <IMediaStreamSourceSwitchStreamsRequestDeferral as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSwitchStreamsRequestDeferral {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestDeferral";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSwitchStreamsRequestDeferral, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSourceSwitchStreamsRequestDeferral {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSwitchStreamsRequestDeferral {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MediaStreamSourceSwitchStreamsRequestedEventArgs(::windows_core::IUnknown);
impl MediaStreamSourceSwitchStreamsRequestedEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<MediaStreamSourceSwitchStreamsRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MediaStreamSourceSwitchStreamsRequestedEventArgs {}
impl ::core::fmt::Debug for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceSwitchStreamsRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestedEventArgs;{42202b72-6ea1-4677-981e-350a0da412aa})");
}
impl ::core::clone::Clone for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    type Vtable = IMediaStreamSourceSwitchStreamsRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    const IID: ::windows_core::GUID = <IMediaStreamSourceSwitchStreamsRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MediaStreamSourceSwitchStreamsRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaStreamSourceSwitchStreamsRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(MediaStreamSourceSwitchStreamsRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MediaStreamSourceSwitchStreamsRequestedEventArgs {}
unsafe impl ::core::marker::Sync for MediaStreamSourceSwitchStreamsRequestedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MseSourceBuffer(::windows_core::IUnknown);
impl MseSourceBuffer {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateStarting<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateStarting)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdateStarting(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdateStarting)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Updated<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Updated)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdated)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateEnded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateEnded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdateEnded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdateEnded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorOccurred<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorOccurred)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveErrorOccurred(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveErrorOccurred)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Aborted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBuffer, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Aborted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAborted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAborted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Mode(&self) -> ::windows_core::Result<MseAppendMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Mode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMode(&self, value: MseAppendMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsUpdating(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUpdating)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Buffered(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MseTimeRange>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Buffered)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TimestampOffset(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimestampOffset)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetTimestampOffset(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTimestampOffset)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppendWindowStart(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendWindowStart)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAppendWindowStart(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppendWindowStart)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AppendWindowEnd(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppendWindowEnd)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAppendWindowEnd<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAppendWindowEnd)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendBuffer<P0>(&self, buffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendBuffer)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendStream<P0>(&self, stream: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendStream)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AppendStreamMaxSize<P0>(&self, stream: P0, maxsize: u64) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IInputStream>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AppendStreamMaxSize)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), maxsize).ok() }
    }
    pub fn Abort(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Abort)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Remove<P0>(&self, start: super::super::Foundation::TimeSpan, end: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), start, end.try_into_param()?.abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for MseSourceBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MseSourceBuffer {}
impl ::core::fmt::Debug for MseSourceBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseSourceBuffer").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MseSourceBuffer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MseSourceBuffer;{0c1aa3e3-df8d-4079-a3fe-6849184b4e2f})");
}
impl ::core::clone::Clone for MseSourceBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MseSourceBuffer {
    type Vtable = IMseSourceBuffer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MseSourceBuffer {
    const IID: ::windows_core::GUID = <IMseSourceBuffer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MseSourceBuffer {
    const NAME: &'static str = "Windows.Media.Core.MseSourceBuffer";
}
::windows_core::imp::interface_hierarchy!(MseSourceBuffer, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MseSourceBuffer {}
unsafe impl ::core::marker::Sync for MseSourceBuffer {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MseSourceBufferList(::windows_core::IUnknown);
impl MseSourceBufferList {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceBufferAdded<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceBufferAdded)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceBufferAdded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceBufferAdded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceBufferRemoved<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseSourceBufferList, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceBufferRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceBufferRemoved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceBufferRemoved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Buffers(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<MseSourceBuffer>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Buffers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for MseSourceBufferList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MseSourceBufferList {}
impl ::core::fmt::Debug for MseSourceBufferList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseSourceBufferList").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MseSourceBufferList {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MseSourceBufferList;{95fae8e7-a8e7-4ebf-8927-145e940ba511})");
}
impl ::core::clone::Clone for MseSourceBufferList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MseSourceBufferList {
    type Vtable = IMseSourceBufferList_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MseSourceBufferList {
    const IID: ::windows_core::GUID = <IMseSourceBufferList as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MseSourceBufferList {
    const NAME: &'static str = "Windows.Media.Core.MseSourceBufferList";
}
::windows_core::imp::interface_hierarchy!(MseSourceBufferList, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for MseSourceBufferList {}
unsafe impl ::core::marker::Sync for MseSourceBufferList {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct MseStreamSource(::windows_core::IUnknown);
impl MseStreamSource {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MseStreamSource, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Opened<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Opened)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOpened(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpened)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Ended<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ended)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<MseStreamSource, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Closed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveClosed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SourceBuffers(&self) -> ::windows_core::Result<MseSourceBufferList> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SourceBuffers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ActiveSourceBuffers(&self) -> ::windows_core::Result<MseSourceBufferList> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActiveSourceBuffers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ReadyState(&self) -> ::windows_core::Result<MseReadyState> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadyState)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn AddSourceBuffer(&self, mimetype: &::windows_core::HSTRING) -> ::windows_core::Result<MseSourceBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddSourceBuffer)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(mimetype), &mut result__).from_abi(result__)
        }
    }
    pub fn RemoveSourceBuffer<P0>(&self, buffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<MseSourceBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSourceBuffer)(::windows_core::Interface::as_raw(this), buffer.into_param().abi()).ok() }
    }
    pub fn EndOfStream(&self, status: MseEndOfStreamStatus) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).EndOfStream)(::windows_core::Interface::as_raw(this), status).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LiveSeekableRange(&self) -> ::windows_core::Result<super::super::Foundation::IReference<MseTimeRange>> {
        let this = &::windows_core::ComInterface::cast::<IMseStreamSource2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LiveSeekableRange)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetLiveSeekableRange<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<MseTimeRange>>,
    {
        let this = &::windows_core::ComInterface::cast::<IMseStreamSource2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLiveSeekableRange)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn IsContentTypeSupported(contenttype: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IMseStreamSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsContentTypeSupported)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(contenttype), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMseStreamSourceStatics<R, F: FnOnce(&IMseStreamSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<MseStreamSource, IMseStreamSourceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for MseStreamSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MseStreamSource {}
impl ::core::fmt::Debug for MseStreamSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseStreamSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MseStreamSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.MseStreamSource;{b0b4198d-02f4-4923-88dd-81bc3f360ffa})");
}
impl ::core::clone::Clone for MseStreamSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for MseStreamSource {
    type Vtable = IMseStreamSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for MseStreamSource {
    const IID: ::windows_core::GUID = <IMseStreamSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for MseStreamSource {
    const NAME: &'static str = "Windows.Media.Core.MseStreamSource";
}
::windows_core::imp::interface_hierarchy!(MseStreamSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaSource> for MseStreamSource {}
unsafe impl ::core::marker::Send for MseStreamSource {}
unsafe impl ::core::marker::Sync for MseStreamSource {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct SceneAnalysisEffect(::windows_core::IUnknown);
impl SceneAnalysisEffect {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.try_into_param()?.abi()).ok() }
    }
    pub fn HighDynamicRangeAnalyzer(&self) -> ::windows_core::Result<HighDynamicRangeControl> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HighDynamicRangeAnalyzer)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredAnalysisInterval(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDesiredAnalysisInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DesiredAnalysisInterval(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DesiredAnalysisInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SceneAnalyzed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<SceneAnalysisEffect, SceneAnalyzedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SceneAnalyzed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSceneAnalyzed(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSceneAnalyzed)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
}
impl ::core::cmp::PartialEq for SceneAnalysisEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneAnalysisEffect {}
impl ::core::fmt::Debug for SceneAnalysisEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisEffect").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneAnalysisEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SceneAnalysisEffect;{c04ba319-ca41-4813-bffd-7b08b0ed2557})");
}
impl ::core::clone::Clone for SceneAnalysisEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneAnalysisEffect {
    type Vtable = ISceneAnalysisEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneAnalysisEffect {
    const IID: ::windows_core::GUID = <ISceneAnalysisEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneAnalysisEffect {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalysisEffect";
}
::windows_core::imp::interface_hierarchy!(SceneAnalysisEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaExtension> for SceneAnalysisEffect {}
unsafe impl ::core::marker::Send for SceneAnalysisEffect {}
unsafe impl ::core::marker::Sync for SceneAnalysisEffect {}
#[doc = "*Required features: `\"Media_Core\"`, `\"Media_Effects\"`*"]
#[cfg(feature = "Media_Effects")]
#[repr(transparent)]
pub struct SceneAnalysisEffectDefinition(::windows_core::IUnknown);
#[cfg(feature = "Media_Effects")]
impl SceneAnalysisEffectDefinition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SceneAnalysisEffectDefinition, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::cmp::PartialEq for SceneAnalysisEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::cmp::Eq for SceneAnalysisEffectDefinition {}
#[cfg(feature = "Media_Effects")]
impl ::core::fmt::Debug for SceneAnalysisEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisEffectDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Media_Effects")]
impl ::windows_core::RuntimeType for SceneAnalysisEffectDefinition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SceneAnalysisEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
}
#[cfg(feature = "Media_Effects")]
impl ::core::clone::Clone for SceneAnalysisEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows_core::Interface for SceneAnalysisEffectDefinition {
    type Vtable = super::Effects::IVideoEffectDefinition_Vtbl;
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows_core::ComInterface for SceneAnalysisEffectDefinition {
    const IID: ::windows_core::GUID = <super::Effects::IVideoEffectDefinition as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Media_Effects")]
impl ::windows_core::RuntimeName for SceneAnalysisEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalysisEffectDefinition";
}
#[cfg(feature = "Media_Effects")]
::windows_core::imp::interface_hierarchy!(SceneAnalysisEffectDefinition, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Media_Effects")]
impl ::windows_core::CanTryInto<super::Effects::IVideoEffectDefinition> for SceneAnalysisEffectDefinition {}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Send for SceneAnalysisEffectDefinition {}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Sync for SceneAnalysisEffectDefinition {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct SceneAnalysisEffectFrame(::windows_core::IUnknown);
impl SceneAnalysisEffectFrame {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Type(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetRelativeTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RelativeTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSystemRelativeTime<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSystemRelativeTime)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SystemRelativeTime(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SystemRelativeTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::IReference<super::super::Foundation::TimeSpan>> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsDiscontinuous(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsDiscontinuous)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDiscontinuous(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDiscontinuous)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ExtendedProperties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = &::windows_core::ComInterface::cast::<super::IMediaFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Capture\"`*"]
    #[cfg(feature = "Media_Capture")]
    pub fn FrameControlValues(&self) -> ::windows_core::Result<super::Capture::CapturedFrameControlValues> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameControlValues)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HighDynamicRange(&self) -> ::windows_core::Result<HighDynamicRangeOutput> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HighDynamicRange)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AnalysisRecommendation(&self) -> ::windows_core::Result<SceneAnalysisRecommendation> {
        let this = &::windows_core::ComInterface::cast::<ISceneAnalysisEffectFrame2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AnalysisRecommendation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SceneAnalysisEffectFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneAnalysisEffectFrame {}
impl ::core::fmt::Debug for SceneAnalysisEffectFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisEffectFrame").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneAnalysisEffectFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SceneAnalysisEffectFrame;{d8b10e4c-7fd9-42e1-85eb-6572c297c987})");
}
impl ::core::clone::Clone for SceneAnalysisEffectFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneAnalysisEffectFrame {
    type Vtable = ISceneAnalysisEffectFrame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneAnalysisEffectFrame {
    const IID: ::windows_core::GUID = <ISceneAnalysisEffectFrame as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneAnalysisEffectFrame {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalysisEffectFrame";
}
::windows_core::imp::interface_hierarchy!(SceneAnalysisEffectFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for SceneAnalysisEffectFrame {}
impl ::windows_core::CanTryInto<super::IMediaFrame> for SceneAnalysisEffectFrame {}
unsafe impl ::core::marker::Send for SceneAnalysisEffectFrame {}
unsafe impl ::core::marker::Sync for SceneAnalysisEffectFrame {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct SceneAnalyzedEventArgs(::windows_core::IUnknown);
impl SceneAnalyzedEventArgs {
    pub fn ResultFrame(&self) -> ::windows_core::Result<SceneAnalysisEffectFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResultFrame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for SceneAnalyzedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SceneAnalyzedEventArgs {}
impl ::core::fmt::Debug for SceneAnalyzedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalyzedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneAnalyzedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SceneAnalyzedEventArgs;{146b9588-2851-45e4-ad55-44cf8df8db4d})");
}
impl ::core::clone::Clone for SceneAnalyzedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SceneAnalyzedEventArgs {
    type Vtable = ISceneAnalyzedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SceneAnalyzedEventArgs {
    const IID: ::windows_core::GUID = <ISceneAnalyzedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SceneAnalyzedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.SceneAnalyzedEventArgs";
}
::windows_core::imp::interface_hierarchy!(SceneAnalyzedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for SceneAnalyzedEventArgs {}
unsafe impl ::core::marker::Sync for SceneAnalyzedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct SpeechCue(::windows_core::IUnknown);
impl SpeechCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SpeechCue, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartPositionInInput(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartPositionInInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartPositionInInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartPositionInInput)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EndPositionInInput(&self) -> ::windows_core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EndPositionInInput)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetEndPositionInInput<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::IReference<i32>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEndPositionInInput)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for SpeechCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SpeechCue {}
impl ::core::fmt::Debug for SpeechCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SpeechCue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SpeechCue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.SpeechCue;{aee254dc-1725-4bad-8043-a98499b017a2})");
}
impl ::core::clone::Clone for SpeechCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SpeechCue {
    type Vtable = ISpeechCue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SpeechCue {
    const IID: ::windows_core::GUID = <ISpeechCue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SpeechCue {
    const NAME: &'static str = "Windows.Media.Core.SpeechCue";
}
::windows_core::imp::interface_hierarchy!(SpeechCue, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaCue> for SpeechCue {}
unsafe impl ::core::marker::Send for SpeechCue {}
unsafe impl ::core::marker::Sync for SpeechCue {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedMetadataStreamDescriptor(::windows_core::IUnknown);
impl TimedMetadataStreamDescriptor {
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::TimedMetadataEncodingProperties> {
        let this = &::windows_core::ComInterface::cast::<ITimedMetadataStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Copy(&self) -> ::windows_core::Result<TimedMetadataStreamDescriptor> {
        let this = &::windows_core::ComInterface::cast::<ITimedMetadataStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Create<P0>(encodingproperties: P0) -> ::windows_core::Result<TimedMetadataStreamDescriptor>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::TimedMetadataEncodingProperties>,
    {
        Self::ITimedMetadataStreamDescriptorFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimedMetadataStreamDescriptorFactory<R, F: FnOnce(&ITimedMetadataStreamDescriptorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedMetadataStreamDescriptor, ITimedMetadataStreamDescriptorFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for TimedMetadataStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataStreamDescriptor {}
impl ::core::fmt::Debug for TimedMetadataStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataStreamDescriptor").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedMetadataStreamDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedMetadataStreamDescriptor;{80f16e6e-92f7-451e-97d2-afd80742da70})");
}
impl ::core::clone::Clone for TimedMetadataStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedMetadataStreamDescriptor {
    type Vtable = IMediaStreamDescriptor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedMetadataStreamDescriptor {
    const IID: ::windows_core::GUID = <IMediaStreamDescriptor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataStreamDescriptor";
}
::windows_core::imp::interface_hierarchy!(TimedMetadataStreamDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaStreamDescriptor> for TimedMetadataStreamDescriptor {}
impl ::windows_core::CanTryInto<IMediaStreamDescriptor2> for TimedMetadataStreamDescriptor {}
unsafe impl ::core::marker::Send for TimedMetadataStreamDescriptor {}
unsafe impl ::core::marker::Sync for TimedMetadataStreamDescriptor {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedMetadataTrack(::windows_core::IUnknown);
impl TimedMetadataTrack {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrackKind(&self) -> ::windows_core::Result<MediaTrackKind> {
        let this = &::windows_core::ComInterface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaTrack>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CueEntered<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CueEntered)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCueEntered(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCueEntered)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CueExited<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, MediaCueEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CueExited)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCueExited(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCueExited)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrackFailed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<TimedMetadataTrack, TimedMetadataTrackFailedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTrackFailed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveTrackFailed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Cues(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<IMediaCue>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Cues)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActiveCues(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<IMediaCue>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActiveCues)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TimedMetadataKind(&self) -> ::windows_core::Result<TimedMetadataKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TimedMetadataKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DispatchType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DispatchType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddCue<P0>(&self, cue: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IMediaCue>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddCue)(::windows_core::Interface::as_raw(this), cue.try_into_param()?.abi()).ok() }
    }
    pub fn RemoveCue<P0>(&self, cue: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<IMediaCue>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCue)(::windows_core::Interface::as_raw(this), cue.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Media_Playback\"`*"]
    #[cfg(feature = "Media_Playback")]
    pub fn PlaybackItem(&self) -> ::windows_core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows_core::ComInterface::cast::<ITimedMetadataTrack2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackItem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<ITimedMetadataTrack2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(id: &::windows_core::HSTRING, language: &::windows_core::HSTRING, kind: TimedMetadataKind) -> ::windows_core::Result<TimedMetadataTrack> {
        Self::ITimedMetadataTrackFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(id), ::core::mem::transmute_copy(language), kind, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimedMetadataTrackFactory<R, F: FnOnce(&ITimedMetadataTrackFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedMetadataTrack, ITimedMetadataTrackFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for TimedMetadataTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataTrack {}
impl ::core::fmt::Debug for TimedMetadataTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrack").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedMetadataTrack {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedMetadataTrack;{9e6aed9e-f67a-49a9-b330-cf03b0e9cf07})");
}
impl ::core::clone::Clone for TimedMetadataTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedMetadataTrack {
    type Vtable = ITimedMetadataTrack_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedMetadataTrack {
    const IID: ::windows_core::GUID = <ITimedMetadataTrack as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataTrack {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrack";
}
::windows_core::imp::interface_hierarchy!(TimedMetadataTrack, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaTrack> for TimedMetadataTrack {}
unsafe impl ::core::marker::Send for TimedMetadataTrack {}
unsafe impl ::core::marker::Sync for TimedMetadataTrack {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedMetadataTrackError(::windows_core::IUnknown);
impl TimedMetadataTrackError {
    pub fn ErrorCode(&self) -> ::windows_core::Result<TimedMetadataTrackErrorCode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ErrorCode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TimedMetadataTrackError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataTrackError {}
impl ::core::fmt::Debug for TimedMetadataTrackError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackError").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedMetadataTrackError {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedMetadataTrackError;{b3767915-4114-4819-b9d9-dd76089e72f8})");
}
impl ::core::clone::Clone for TimedMetadataTrackError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedMetadataTrackError {
    type Vtable = ITimedMetadataTrackError_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedMetadataTrackError {
    const IID: ::windows_core::GUID = <ITimedMetadataTrackError as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataTrackError {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrackError";
}
::windows_core::imp::interface_hierarchy!(TimedMetadataTrackError, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TimedMetadataTrackError {}
unsafe impl ::core::marker::Sync for TimedMetadataTrackError {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedMetadataTrackFailedEventArgs(::windows_core::IUnknown);
impl TimedMetadataTrackFailedEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<TimedMetadataTrackError> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TimedMetadataTrackFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedMetadataTrackFailedEventArgs {}
impl ::core::fmt::Debug for TimedMetadataTrackFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackFailedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedMetadataTrackFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedMetadataTrackFailedEventArgs;{a57fc9d1-6789-4d4d-b07f-84b4f31acb70})");
}
impl ::core::clone::Clone for TimedMetadataTrackFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedMetadataTrackFailedEventArgs {
    type Vtable = ITimedMetadataTrackFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedMetadataTrackFailedEventArgs {
    const IID: ::windows_core::GUID = <ITimedMetadataTrackFailedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedMetadataTrackFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrackFailedEventArgs";
}
::windows_core::imp::interface_hierarchy!(TimedMetadataTrackFailedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TimedMetadataTrackFailedEventArgs {}
unsafe impl ::core::marker::Sync for TimedMetadataTrackFailedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextBouten(::windows_core::IUnknown);
impl TimedTextBouten {
    pub fn Type(&self) -> ::windows_core::Result<TimedTextBoutenType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetType(&self, value: TimedTextBoutenType) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetType)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Position(&self) -> ::windows_core::Result<TimedTextBoutenPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPosition(&self, value: TimedTextBoutenPosition) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for TimedTextBouten {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextBouten {}
impl ::core::fmt::Debug for TimedTextBouten {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextBouten").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextBouten {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextBouten;{d9062783-5597-5092-820c-8f738e0f774a})");
}
impl ::core::clone::Clone for TimedTextBouten {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedTextBouten {
    type Vtable = ITimedTextBouten_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedTextBouten {
    const IID: ::windows_core::GUID = <ITimedTextBouten as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextBouten {
    const NAME: &'static str = "Windows.Media.Core.TimedTextBouten";
}
::windows_core::imp::interface_hierarchy!(TimedTextBouten, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TimedTextBouten {}
unsafe impl ::core::marker::Sync for TimedTextBouten {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextCue(::windows_core::IUnknown);
impl TimedTextCue {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextCue, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartTime(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetStartTime)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartTime(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartTime)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetId)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaCue>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CueRegion(&self) -> ::windows_core::Result<TimedTextRegion> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CueRegion)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCueRegion<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<TimedTextRegion>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCueRegion)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn CueStyle(&self) -> ::windows_core::Result<TimedTextStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CueStyle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCueStyle<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<TimedTextStyle>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCueStyle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lines(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<TimedTextLine>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lines)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TimedTextCue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextCue {}
impl ::core::fmt::Debug for TimedTextCue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextCue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextCue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextCue;{51c79e51-3b86-494d-b359-bb2ea7aca9a9})");
}
impl ::core::clone::Clone for TimedTextCue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedTextCue {
    type Vtable = ITimedTextCue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedTextCue {
    const IID: ::windows_core::GUID = <ITimedTextCue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextCue {
    const NAME: &'static str = "Windows.Media.Core.TimedTextCue";
}
::windows_core::imp::interface_hierarchy!(TimedTextCue, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaCue> for TimedTextCue {}
unsafe impl ::core::marker::Send for TimedTextCue {}
unsafe impl ::core::marker::Sync for TimedTextCue {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextLine(::windows_core::IUnknown);
impl TimedTextLine {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextLine, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Subformats(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<TimedTextSubformat>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Subformats)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TimedTextLine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextLine {}
impl ::core::fmt::Debug for TimedTextLine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextLine").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextLine {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextLine;{978d7ce2-7308-4c66-be50-65777289f5df})");
}
impl ::core::clone::Clone for TimedTextLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedTextLine {
    type Vtable = ITimedTextLine_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedTextLine {
    const IID: ::windows_core::GUID = <ITimedTextLine as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextLine {
    const NAME: &'static str = "Windows.Media.Core.TimedTextLine";
}
::windows_core::imp::interface_hierarchy!(TimedTextLine, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TimedTextLine {}
unsafe impl ::core::marker::Sync for TimedTextLine {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextRegion(::windows_core::IUnknown);
impl TimedTextRegion {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextRegion, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Position(&self) -> ::windows_core::Result<TimedTextPoint> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPosition(&self, value: TimedTextPoint) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Extent(&self) -> ::windows_core::Result<TimedTextSize> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Extent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetExtent(&self, value: TimedTextSize) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetExtent)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn Background(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Background)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetBackground(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackground)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WritingMode(&self) -> ::windows_core::Result<TimedTextWritingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WritingMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWritingMode(&self, value: TimedTextWritingMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWritingMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DisplayAlignment(&self) -> ::windows_core::Result<TimedTextDisplayAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayAlignment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDisplayAlignment(&self, value: TimedTextDisplayAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDisplayAlignment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineHeight(&self) -> ::windows_core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLineHeight(&self, value: TimedTextDouble) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLineHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsOverflowClipped(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOverflowClipped)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsOverflowClipped(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsOverflowClipped)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Padding(&self) -> ::windows_core::Result<TimedTextPadding> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Padding)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPadding(&self, value: TimedTextPadding) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPadding)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TextWrapping(&self) -> ::windows_core::Result<TimedTextWrapping> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextWrapping)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTextWrapping(&self, value: TimedTextWrapping) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetTextWrapping)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScrollMode(&self) -> ::windows_core::Result<TimedTextScrollMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScrollMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetScrollMode(&self, value: TimedTextScrollMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScrollMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for TimedTextRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextRegion {}
impl ::core::fmt::Debug for TimedTextRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRegion").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextRegion {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextRegion;{1ed0881f-8a06-4222-9f59-b21bf40124b4})");
}
impl ::core::clone::Clone for TimedTextRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedTextRegion {
    type Vtable = ITimedTextRegion_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedTextRegion {
    const IID: ::windows_core::GUID = <ITimedTextRegion as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextRegion {
    const NAME: &'static str = "Windows.Media.Core.TimedTextRegion";
}
::windows_core::imp::interface_hierarchy!(TimedTextRegion, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TimedTextRegion {}
unsafe impl ::core::marker::Sync for TimedTextRegion {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextRuby(::windows_core::IUnknown);
impl TimedTextRuby {
    pub fn Text(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Text)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetText)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Position(&self) -> ::windows_core::Result<TimedTextRubyPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPosition(&self, value: TimedTextRubyPosition) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPosition)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Align(&self) -> ::windows_core::Result<TimedTextRubyAlign> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Align)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetAlign(&self, value: TimedTextRubyAlign) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAlign)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Reserve(&self) -> ::windows_core::Result<TimedTextRubyReserve> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reserve)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetReserve(&self, value: TimedTextRubyReserve) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetReserve)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for TimedTextRuby {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextRuby {}
impl ::core::fmt::Debug for TimedTextRuby {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRuby").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextRuby {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextRuby;{10335c29-5b3c-5693-9959-d05a0bd24628})");
}
impl ::core::clone::Clone for TimedTextRuby {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedTextRuby {
    type Vtable = ITimedTextRuby_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedTextRuby {
    const IID: ::windows_core::GUID = <ITimedTextRuby as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextRuby {
    const NAME: &'static str = "Windows.Media.Core.TimedTextRuby";
}
::windows_core::imp::interface_hierarchy!(TimedTextRuby, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TimedTextRuby {}
unsafe impl ::core::marker::Sync for TimedTextRuby {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextSource(::windows_core::IUnknown);
impl TimedTextSource {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Resolved<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<TimedTextSource, TimedTextSourceResolveResultEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Resolved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResolved(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveResolved)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStream<P0>(stream: P0) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStream)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUri<P0>(uri: P0) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUri)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamWithLanguage<P0>(stream: P0, defaultlanguage: &::windows_core::HSTRING) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamWithLanguage)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), ::core::mem::transmute_copy(defaultlanguage), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUriWithLanguage<P0>(uri: P0, defaultlanguage: &::windows_core::HSTRING) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUriWithLanguage)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), ::core::mem::transmute_copy(defaultlanguage), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamWithIndex<P0, P1>(stream: P0, indexstream: P1) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamWithIndex)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), indexstream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUriWithIndex<P0, P1>(uri: P0, indexuri: P1) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUriWithIndex)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), indexuri.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamWithIndexAndLanguage<P0, P1>(stream: P0, indexstream: P1, defaultlanguage: &::windows_core::HSTRING) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
        P1: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromStreamWithIndexAndLanguage)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), indexstream.try_into_param()?.abi(), ::core::mem::transmute_copy(defaultlanguage), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFromUriWithIndexAndLanguage<P0, P1>(uri: P0, indexuri: P1, defaultlanguage: &::windows_core::HSTRING) -> ::windows_core::Result<TimedTextSource>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::Uri>,
        P1: ::windows_core::IntoParam<super::super::Foundation::Uri>,
    {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateFromUriWithIndexAndLanguage)(::windows_core::Interface::as_raw(this), uri.into_param().abi(), indexuri.into_param().abi(), ::core::mem::transmute_copy(defaultlanguage), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ITimedTextSourceStatics<R, F: FnOnce(&ITimedTextSourceStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextSource, ITimedTextSourceStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ITimedTextSourceStatics2<R, F: FnOnce(&ITimedTextSourceStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextSource, ITimedTextSourceStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for TimedTextSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextSource {}
impl ::core::fmt::Debug for TimedTextSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextSource").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextSource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextSource;{c4ed9ba6-101f-404d-a949-82f33fcd93b7})");
}
impl ::core::clone::Clone for TimedTextSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedTextSource {
    type Vtable = ITimedTextSource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedTextSource {
    const IID: ::windows_core::GUID = <ITimedTextSource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextSource {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSource";
}
::windows_core::imp::interface_hierarchy!(TimedTextSource, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TimedTextSource {}
unsafe impl ::core::marker::Sync for TimedTextSource {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextSourceResolveResultEventArgs(::windows_core::IUnknown);
impl TimedTextSourceResolveResultEventArgs {
    pub fn Error(&self) -> ::windows_core::Result<TimedMetadataTrackError> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Error)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tracks(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<TimedMetadataTrack>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Tracks)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for TimedTextSourceResolveResultEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextSourceResolveResultEventArgs {}
impl ::core::fmt::Debug for TimedTextSourceResolveResultEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextSourceResolveResultEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextSourceResolveResultEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextSourceResolveResultEventArgs;{48907c9c-dcd8-4c33-9ad3-6cdce7b1c566})");
}
impl ::core::clone::Clone for TimedTextSourceResolveResultEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedTextSourceResolveResultEventArgs {
    type Vtable = ITimedTextSourceResolveResultEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedTextSourceResolveResultEventArgs {
    const IID: ::windows_core::GUID = <ITimedTextSourceResolveResultEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextSourceResolveResultEventArgs {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSourceResolveResultEventArgs";
}
::windows_core::imp::interface_hierarchy!(TimedTextSourceResolveResultEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TimedTextSourceResolveResultEventArgs {}
unsafe impl ::core::marker::Sync for TimedTextSourceResolveResultEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextStyle(::windows_core::IUnknown);
impl TimedTextStyle {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextStyle, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn FontFamily(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontFamily)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFontFamily(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFontFamily)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn FontSize(&self) -> ::windows_core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: TimedTextDouble) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFontSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontWeight(&self) -> ::windows_core::Result<TimedTextWeight> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontWeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFontWeight(&self, value: TimedTextWeight) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFontWeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn Foreground(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Foreground)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetForeground(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetForeground)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn Background(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Background)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetBackground(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBackground)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsBackgroundAlwaysShown(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsBackgroundAlwaysShown)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsBackgroundAlwaysShown(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsBackgroundAlwaysShown)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FlowDirection(&self) -> ::windows_core::Result<TimedTextFlowDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlowDirection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFlowDirection(&self, value: TimedTextFlowDirection) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFlowDirection)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LineAlignment(&self) -> ::windows_core::Result<TimedTextLineAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LineAlignment)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLineAlignment(&self, value: TimedTextLineAlignment) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLineAlignment)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn OutlineColor(&self) -> ::windows_core::Result<super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OutlineColor)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetOutlineColor(&self, value: super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutlineColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutlineThickness(&self) -> ::windows_core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OutlineThickness)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutlineThickness(&self, value: TimedTextDouble) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutlineThickness)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn OutlineRadius(&self) -> ::windows_core::Result<TimedTextDouble> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OutlineRadius)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOutlineRadius(&self, value: TimedTextDouble) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOutlineRadius)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStyle(&self) -> ::windows_core::Result<TimedTextFontStyle> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontStyle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFontStyle(&self, value: TimedTextFontStyle) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFontStyle)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsUnderlineEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsUnderlineEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsUnderlineEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsUnderlineEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsLineThroughEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsLineThroughEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsLineThroughEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsLineThroughEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsOverlineEnabled(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsOverlineEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsOverlineEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsOverlineEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Ruby(&self) -> ::windows_core::Result<TimedTextRuby> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Ruby)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Bouten(&self) -> ::windows_core::Result<TimedTextBouten> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bouten)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsTextCombined(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsTextCombined)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsTextCombined(&self, value: bool) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetIsTextCombined)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontAngleInDegrees(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FontAngleInDegrees)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFontAngleInDegrees(&self, value: f64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ITimedTextStyle3>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetFontAngleInDegrees)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for TimedTextStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextStyle {}
impl ::core::fmt::Debug for TimedTextStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextStyle").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextStyle {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextStyle;{1bb2384d-a825-40c2-a7f5-281eaedf3b55})");
}
impl ::core::clone::Clone for TimedTextStyle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedTextStyle {
    type Vtable = ITimedTextStyle_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedTextStyle {
    const IID: ::windows_core::GUID = <ITimedTextStyle as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextStyle {
    const NAME: &'static str = "Windows.Media.Core.TimedTextStyle";
}
::windows_core::imp::interface_hierarchy!(TimedTextStyle, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TimedTextStyle {}
unsafe impl ::core::marker::Sync for TimedTextStyle {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct TimedTextSubformat(::windows_core::IUnknown);
impl TimedTextSubformat {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<TimedTextSubformat, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn StartIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetStartIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Length(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Length)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLength(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLength)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SubformatStyle(&self) -> ::windows_core::Result<TimedTextStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SubformatStyle)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSubformatStyle<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<TimedTextStyle>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSubformatStyle)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for TimedTextSubformat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimedTextSubformat {}
impl ::core::fmt::Debug for TimedTextSubformat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextSubformat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextSubformat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.TimedTextSubformat;{d713502f-3261-4722-a0c2-b937b2390f14})");
}
impl ::core::clone::Clone for TimedTextSubformat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for TimedTextSubformat {
    type Vtable = ITimedTextSubformat_Vtbl;
}
unsafe impl ::windows_core::ComInterface for TimedTextSubformat {
    const IID: ::windows_core::GUID = <ITimedTextSubformat as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for TimedTextSubformat {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSubformat";
}
::windows_core::imp::interface_hierarchy!(TimedTextSubformat, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for TimedTextSubformat {}
unsafe impl ::core::marker::Sync for TimedTextSubformat {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct VideoStabilizationEffect(::windows_core::IUnknown);
impl VideoStabilizationEffect {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetProperties<P0>(&self, configuration: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = &::windows_core::ComInterface::cast::<super::IMediaExtension>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetProperties)(::windows_core::Interface::as_raw(this), configuration.try_into_param()?.abi()).ok() }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Enabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Enabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnabledChanged<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VideoStabilizationEffect, VideoStabilizationEffectEnabledChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnabledChanged)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnabledChanged(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnabledChanged)(::windows_core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"Media_Capture\"`, `\"Media_Devices\"`, `\"Media_MediaProperties\"`*"]
    #[cfg(all(feature = "Media_Capture", feature = "Media_Devices", feature = "Media_MediaProperties"))]
    pub fn GetRecommendedStreamConfiguration<P0, P1>(&self, controller: P0, desiredproperties: P1) -> ::windows_core::Result<super::Capture::VideoStreamConfiguration>
    where
        P0: ::windows_core::IntoParam<super::Devices::VideoDeviceController>,
        P1: ::windows_core::IntoParam<super::MediaProperties::VideoEncodingProperties>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetRecommendedStreamConfiguration)(::windows_core::Interface::as_raw(this), controller.into_param().abi(), desiredproperties.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VideoStabilizationEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoStabilizationEffect {}
impl ::core::fmt::Debug for VideoStabilizationEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffect").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoStabilizationEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoStabilizationEffect;{0808a650-9698-4e57-877b-bd7cb2ee0f8a})");
}
impl ::core::clone::Clone for VideoStabilizationEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VideoStabilizationEffect {
    type Vtable = IVideoStabilizationEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoStabilizationEffect {
    const IID: ::windows_core::GUID = <IVideoStabilizationEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoStabilizationEffect {
    const NAME: &'static str = "Windows.Media.Core.VideoStabilizationEffect";
}
::windows_core::imp::interface_hierarchy!(VideoStabilizationEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<super::IMediaExtension> for VideoStabilizationEffect {}
unsafe impl ::core::marker::Send for VideoStabilizationEffect {}
unsafe impl ::core::marker::Sync for VideoStabilizationEffect {}
#[doc = "*Required features: `\"Media_Core\"`, `\"Media_Effects\"`*"]
#[cfg(feature = "Media_Effects")]
#[repr(transparent)]
pub struct VideoStabilizationEffectDefinition(::windows_core::IUnknown);
#[cfg(feature = "Media_Effects")]
impl VideoStabilizationEffectDefinition {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VideoStabilizationEffectDefinition, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Media_Effects\"`*"]
    #[cfg(feature = "Media_Effects")]
    pub fn ActivatableClassId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ActivatableClassId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Media_Effects\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Media_Effects"))]
    pub fn Properties(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Properties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::cmp::PartialEq for VideoStabilizationEffectDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Media_Effects")]
impl ::core::cmp::Eq for VideoStabilizationEffectDefinition {}
#[cfg(feature = "Media_Effects")]
impl ::core::fmt::Debug for VideoStabilizationEffectDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffectDefinition").field(&self.0).finish()
    }
}
#[cfg(feature = "Media_Effects")]
impl ::windows_core::RuntimeType for VideoStabilizationEffectDefinition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoStabilizationEffectDefinition;{39f38cf0-8d0f-4f3e-84fc-2d46a5297943})");
}
#[cfg(feature = "Media_Effects")]
impl ::core::clone::Clone for VideoStabilizationEffectDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows_core::Interface for VideoStabilizationEffectDefinition {
    type Vtable = super::Effects::IVideoEffectDefinition_Vtbl;
}
#[cfg(feature = "Media_Effects")]
unsafe impl ::windows_core::ComInterface for VideoStabilizationEffectDefinition {
    const IID: ::windows_core::GUID = <super::Effects::IVideoEffectDefinition as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Media_Effects")]
impl ::windows_core::RuntimeName for VideoStabilizationEffectDefinition {
    const NAME: &'static str = "Windows.Media.Core.VideoStabilizationEffectDefinition";
}
#[cfg(feature = "Media_Effects")]
::windows_core::imp::interface_hierarchy!(VideoStabilizationEffectDefinition, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Media_Effects")]
impl ::windows_core::CanTryInto<super::Effects::IVideoEffectDefinition> for VideoStabilizationEffectDefinition {}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Send for VideoStabilizationEffectDefinition {}
#[cfg(feature = "Media_Effects")]
unsafe impl ::core::marker::Sync for VideoStabilizationEffectDefinition {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct VideoStabilizationEffectEnabledChangedEventArgs(::windows_core::IUnknown);
impl VideoStabilizationEffectEnabledChangedEventArgs {
    pub fn Reason(&self) -> ::windows_core::Result<VideoStabilizationEffectEnabledChangedReason> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Reason)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VideoStabilizationEffectEnabledChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoStabilizationEffectEnabledChangedEventArgs {}
impl ::core::fmt::Debug for VideoStabilizationEffectEnabledChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffectEnabledChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoStabilizationEffectEnabledChangedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoStabilizationEffectEnabledChangedEventArgs;{187eff28-67bb-4713-b900-4168da164529})");
}
impl ::core::clone::Clone for VideoStabilizationEffectEnabledChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VideoStabilizationEffectEnabledChangedEventArgs {
    type Vtable = IVideoStabilizationEffectEnabledChangedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoStabilizationEffectEnabledChangedEventArgs {
    const IID: ::windows_core::GUID = <IVideoStabilizationEffectEnabledChangedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoStabilizationEffectEnabledChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.VideoStabilizationEffectEnabledChangedEventArgs";
}
::windows_core::imp::interface_hierarchy!(VideoStabilizationEffectEnabledChangedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VideoStabilizationEffectEnabledChangedEventArgs {}
unsafe impl ::core::marker::Sync for VideoStabilizationEffectEnabledChangedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct VideoStreamDescriptor(::windows_core::IUnknown);
impl VideoStreamDescriptor {
    pub fn IsSelected(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsSelected)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLanguage(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IMediaStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn EncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EncodingProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Copy(&self) -> ::windows_core::Result<VideoStreamDescriptor> {
        let this = &::windows_core::ComInterface::cast::<IVideoStreamDescriptor2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn Create<P0>(encodingproperties: P0) -> ::windows_core::Result<VideoStreamDescriptor>
    where
        P0: ::windows_core::IntoParam<super::MediaProperties::VideoEncodingProperties>,
    {
        Self::IVideoStreamDescriptorFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), encodingproperties.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IVideoStreamDescriptorFactory<R, F: FnOnce(&IVideoStreamDescriptorFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<VideoStreamDescriptor, IVideoStreamDescriptorFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for VideoStreamDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoStreamDescriptor {}
impl ::core::fmt::Debug for VideoStreamDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStreamDescriptor").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoStreamDescriptor {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoStreamDescriptor;{12ee0d55-9c2b-4440-8057-2c7a90f0cbec})");
}
impl ::core::clone::Clone for VideoStreamDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VideoStreamDescriptor {
    type Vtable = IVideoStreamDescriptor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoStreamDescriptor {
    const IID: ::windows_core::GUID = <IVideoStreamDescriptor as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoStreamDescriptor {
    const NAME: &'static str = "Windows.Media.Core.VideoStreamDescriptor";
}
::windows_core::imp::interface_hierarchy!(VideoStreamDescriptor, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaStreamDescriptor> for VideoStreamDescriptor {}
impl ::windows_core::CanTryInto<IMediaStreamDescriptor2> for VideoStreamDescriptor {}
unsafe impl ::core::marker::Send for VideoStreamDescriptor {}
unsafe impl ::core::marker::Sync for VideoStreamDescriptor {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct VideoTrack(::windows_core::IUnknown);
impl VideoTrack {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Language(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Language)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn TrackKind(&self) -> ::windows_core::Result<MediaTrackKind> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TrackKind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLabel)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Label)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenFailed<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<VideoTrack, VideoTrackOpenFailedEventArgs>>,
    {
        let this = &::windows_core::ComInterface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OpenFailed)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOpenFailed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IVideoTrack>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).RemoveOpenFailed)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Media_MediaProperties\"`*"]
    #[cfg(feature = "Media_MediaProperties")]
    pub fn GetEncodingProperties(&self) -> ::windows_core::Result<super::MediaProperties::VideoEncodingProperties> {
        let this = &::windows_core::ComInterface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEncodingProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Media_Playback\"`*"]
    #[cfg(feature = "Media_Playback")]
    pub fn PlaybackItem(&self) -> ::windows_core::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows_core::ComInterface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PlaybackItem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Name)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SupportInfo(&self) -> ::windows_core::Result<VideoTrackSupportInfo> {
        let this = &::windows_core::ComInterface::cast::<IVideoTrack>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SupportInfo)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VideoTrack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTrack {}
impl ::core::fmt::Debug for VideoTrack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTrack").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoTrack {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoTrack;{03e1fafc-c931-491a-b46b-c10ee8c256b7})");
}
impl ::core::clone::Clone for VideoTrack {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VideoTrack {
    type Vtable = IMediaTrack_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoTrack {
    const IID: ::windows_core::GUID = <IMediaTrack as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoTrack {
    const NAME: &'static str = "Windows.Media.Core.VideoTrack";
}
::windows_core::imp::interface_hierarchy!(VideoTrack, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IMediaTrack> for VideoTrack {}
unsafe impl ::core::marker::Send for VideoTrack {}
unsafe impl ::core::marker::Sync for VideoTrack {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct VideoTrackOpenFailedEventArgs(::windows_core::IUnknown);
impl VideoTrackOpenFailedEventArgs {
    pub fn ExtendedError(&self) -> ::windows_core::Result<::windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExtendedError)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VideoTrackOpenFailedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTrackOpenFailedEventArgs {}
impl ::core::fmt::Debug for VideoTrackOpenFailedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTrackOpenFailedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoTrackOpenFailedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoTrackOpenFailedEventArgs;{7679e231-04f9-4c82-a4ee-8602c8bb4754})");
}
impl ::core::clone::Clone for VideoTrackOpenFailedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VideoTrackOpenFailedEventArgs {
    type Vtable = IVideoTrackOpenFailedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoTrackOpenFailedEventArgs {
    const IID: ::windows_core::GUID = <IVideoTrackOpenFailedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoTrackOpenFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.VideoTrackOpenFailedEventArgs";
}
::windows_core::imp::interface_hierarchy!(VideoTrackOpenFailedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VideoTrackOpenFailedEventArgs {}
unsafe impl ::core::marker::Sync for VideoTrackOpenFailedEventArgs {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
pub struct VideoTrackSupportInfo(::windows_core::IUnknown);
impl VideoTrackSupportInfo {
    pub fn DecoderStatus(&self) -> ::windows_core::Result<MediaDecoderStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecoderStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MediaSourceStatus(&self) -> ::windows_core::Result<MediaSourceStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MediaSourceStatus)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VideoTrackSupportInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VideoTrackSupportInfo {}
impl ::core::fmt::Debug for VideoTrackSupportInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoTrackSupportInfo").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoTrackSupportInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Core.VideoTrackSupportInfo;{4bb534a0-fc5f-450d-8ff0-778d590486de})");
}
impl ::core::clone::Clone for VideoTrackSupportInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VideoTrackSupportInfo {
    type Vtable = IVideoTrackSupportInfo_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VideoTrackSupportInfo {
    const IID: ::windows_core::GUID = <IVideoTrackSupportInfo as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VideoTrackSupportInfo {
    const NAME: &'static str = "Windows.Media.Core.VideoTrackSupportInfo";
}
::windows_core::imp::interface_hierarchy!(VideoTrackSupportInfo, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VideoTrackSupportInfo {}
unsafe impl ::core::marker::Sync for VideoTrackSupportInfo {}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioDecoderDegradation(pub i32);
impl AudioDecoderDegradation {
    pub const None: Self = Self(0i32);
    pub const DownmixTo2Channels: Self = Self(1i32);
    pub const DownmixTo6Channels: Self = Self(2i32);
    pub const DownmixTo8Channels: Self = Self(3i32);
}
impl ::core::marker::Copy for AudioDecoderDegradation {}
impl ::core::clone::Clone for AudioDecoderDegradation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioDecoderDegradation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AudioDecoderDegradation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AudioDecoderDegradation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDecoderDegradation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioDecoderDegradation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.AudioDecoderDegradation;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AudioDecoderDegradationReason(pub i32);
impl AudioDecoderDegradationReason {
    pub const None: Self = Self(0i32);
    pub const LicensingRequirement: Self = Self(1i32);
    pub const SpatialAudioNotSupported: Self = Self(2i32);
}
impl ::core::marker::Copy for AudioDecoderDegradationReason {}
impl ::core::clone::Clone for AudioDecoderDegradationReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AudioDecoderDegradationReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AudioDecoderDegradationReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AudioDecoderDegradationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AudioDecoderDegradationReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AudioDecoderDegradationReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.AudioDecoderDegradationReason;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CodecCategory(pub i32);
impl CodecCategory {
    pub const Encoder: Self = Self(0i32);
    pub const Decoder: Self = Self(1i32);
}
impl ::core::marker::Copy for CodecCategory {}
impl ::core::clone::Clone for CodecCategory {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CodecCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CodecCategory {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CodecCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecCategory").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CodecCategory {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.CodecCategory;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CodecKind(pub i32);
impl CodecKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
}
impl ::core::marker::Copy for CodecKind {}
impl ::core::clone::Clone for CodecKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CodecKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for CodecKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CodecKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CodecKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for CodecKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.CodecKind;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FaceDetectionMode(pub i32);
impl FaceDetectionMode {
    pub const HighPerformance: Self = Self(0i32);
    pub const Balanced: Self = Self(1i32);
    pub const HighQuality: Self = Self(2i32);
}
impl ::core::marker::Copy for FaceDetectionMode {}
impl ::core::clone::Clone for FaceDetectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FaceDetectionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for FaceDetectionMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for FaceDetectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FaceDetectionMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for FaceDetectionMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.FaceDetectionMode;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaDecoderStatus(pub i32);
impl MediaDecoderStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const UnsupportedSubtype: Self = Self(1i32);
    pub const UnsupportedEncoderProperties: Self = Self(2i32);
    pub const Degraded: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaDecoderStatus {}
impl ::core::clone::Clone for MediaDecoderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaDecoderStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaDecoderStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaDecoderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaDecoderStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaDecoderStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaDecoderStatus;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaSourceState(pub i32);
impl MediaSourceState {
    pub const Initial: Self = Self(0i32);
    pub const Opening: Self = Self(1i32);
    pub const Opened: Self = Self(2i32);
    pub const Failed: Self = Self(3i32);
    pub const Closed: Self = Self(4i32);
}
impl ::core::marker::Copy for MediaSourceState {}
impl ::core::clone::Clone for MediaSourceState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaSourceState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaSourceState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaSourceState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaSourceState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaSourceState;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaSourceStatus(pub i32);
impl MediaSourceStatus {
    pub const FullySupported: Self = Self(0i32);
    pub const Unknown: Self = Self(1i32);
}
impl ::core::marker::Copy for MediaSourceStatus {}
impl ::core::clone::Clone for MediaSourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaSourceStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaSourceStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaSourceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaSourceStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaSourceStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaSourceStatus;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaStreamSourceClosedReason(pub i32);
impl MediaStreamSourceClosedReason {
    pub const Done: Self = Self(0i32);
    pub const UnknownError: Self = Self(1i32);
    pub const AppReportedError: Self = Self(2i32);
    pub const UnsupportedProtectionSystem: Self = Self(3i32);
    pub const ProtectionSystemFailure: Self = Self(4i32);
    pub const UnsupportedEncodingFormat: Self = Self(5i32);
    pub const MissingSampleRequestedEventHandler: Self = Self(6i32);
}
impl ::core::marker::Copy for MediaStreamSourceClosedReason {}
impl ::core::clone::Clone for MediaStreamSourceClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaStreamSourceClosedReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaStreamSourceClosedReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaStreamSourceClosedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceClosedReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceClosedReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaStreamSourceClosedReason;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaStreamSourceErrorStatus(pub i32);
impl MediaStreamSourceErrorStatus {
    pub const Other: Self = Self(0i32);
    pub const OutOfMemory: Self = Self(1i32);
    pub const FailedToOpenFile: Self = Self(2i32);
    pub const FailedToConnectToServer: Self = Self(3i32);
    pub const ConnectionToServerLost: Self = Self(4i32);
    pub const UnspecifiedNetworkError: Self = Self(5i32);
    pub const DecodeError: Self = Self(6i32);
    pub const UnsupportedMediaFormat: Self = Self(7i32);
}
impl ::core::marker::Copy for MediaStreamSourceErrorStatus {}
impl ::core::clone::Clone for MediaStreamSourceErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaStreamSourceErrorStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaStreamSourceErrorStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaStreamSourceErrorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaStreamSourceErrorStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaStreamSourceErrorStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaStreamSourceErrorStatus;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MediaTrackKind(pub i32);
impl MediaTrackKind {
    pub const Audio: Self = Self(0i32);
    pub const Video: Self = Self(1i32);
    pub const TimedMetadata: Self = Self(2i32);
}
impl ::core::marker::Copy for MediaTrackKind {}
impl ::core::clone::Clone for MediaTrackKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MediaTrackKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MediaTrackKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MediaTrackKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MediaTrackKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MediaTrackKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MediaTrackKind;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MseAppendMode(pub i32);
impl MseAppendMode {
    pub const Segments: Self = Self(0i32);
    pub const Sequence: Self = Self(1i32);
}
impl ::core::marker::Copy for MseAppendMode {}
impl ::core::clone::Clone for MseAppendMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MseAppendMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MseAppendMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MseAppendMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseAppendMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MseAppendMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MseAppendMode;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MseEndOfStreamStatus(pub i32);
impl MseEndOfStreamStatus {
    pub const Success: Self = Self(0i32);
    pub const NetworkError: Self = Self(1i32);
    pub const DecodeError: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for MseEndOfStreamStatus {}
impl ::core::clone::Clone for MseEndOfStreamStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MseEndOfStreamStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MseEndOfStreamStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MseEndOfStreamStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseEndOfStreamStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MseEndOfStreamStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MseEndOfStreamStatus;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MseReadyState(pub i32);
impl MseReadyState {
    pub const Closed: Self = Self(0i32);
    pub const Open: Self = Self(1i32);
    pub const Ended: Self = Self(2i32);
}
impl ::core::marker::Copy for MseReadyState {}
impl ::core::clone::Clone for MseReadyState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MseReadyState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for MseReadyState {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for MseReadyState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MseReadyState").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for MseReadyState {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.MseReadyState;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SceneAnalysisRecommendation(pub i32);
impl SceneAnalysisRecommendation {
    pub const Standard: Self = Self(0i32);
    pub const Hdr: Self = Self(1i32);
    pub const LowLight: Self = Self(2i32);
}
impl ::core::marker::Copy for SceneAnalysisRecommendation {}
impl ::core::clone::Clone for SceneAnalysisRecommendation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SceneAnalysisRecommendation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SceneAnalysisRecommendation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SceneAnalysisRecommendation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SceneAnalysisRecommendation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SceneAnalysisRecommendation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.SceneAnalysisRecommendation;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedMetadataKind(pub i32);
impl TimedMetadataKind {
    pub const Caption: Self = Self(0i32);
    pub const Chapter: Self = Self(1i32);
    pub const Custom: Self = Self(2i32);
    pub const Data: Self = Self(3i32);
    pub const Description: Self = Self(4i32);
    pub const Subtitle: Self = Self(5i32);
    pub const ImageSubtitle: Self = Self(6i32);
    pub const Speech: Self = Self(7i32);
}
impl ::core::marker::Copy for TimedMetadataKind {}
impl ::core::clone::Clone for TimedMetadataKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedMetadataKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedMetadataKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedMetadataKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedMetadataKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedMetadataKind;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedMetadataTrackErrorCode(pub i32);
impl TimedMetadataTrackErrorCode {
    pub const None: Self = Self(0i32);
    pub const DataFormatError: Self = Self(1i32);
    pub const NetworkError: Self = Self(2i32);
    pub const InternalError: Self = Self(3i32);
}
impl ::core::marker::Copy for TimedMetadataTrackErrorCode {}
impl ::core::clone::Clone for TimedMetadataTrackErrorCode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedMetadataTrackErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedMetadataTrackErrorCode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedMetadataTrackErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedMetadataTrackErrorCode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedMetadataTrackErrorCode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedMetadataTrackErrorCode;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextBoutenPosition(pub i32);
impl TimedTextBoutenPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextBoutenPosition {}
impl ::core::clone::Clone for TimedTextBoutenPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextBoutenPosition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextBoutenPosition {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextBoutenPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextBoutenPosition").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextBoutenPosition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextBoutenPosition;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextBoutenType(pub i32);
impl TimedTextBoutenType {
    pub const None: Self = Self(0i32);
    pub const Auto: Self = Self(1i32);
    pub const FilledCircle: Self = Self(2i32);
    pub const OpenCircle: Self = Self(3i32);
    pub const FilledDot: Self = Self(4i32);
    pub const OpenDot: Self = Self(5i32);
    pub const FilledSesame: Self = Self(6i32);
    pub const OpenSesame: Self = Self(7i32);
}
impl ::core::marker::Copy for TimedTextBoutenType {}
impl ::core::clone::Clone for TimedTextBoutenType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextBoutenType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextBoutenType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextBoutenType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextBoutenType").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextBoutenType {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextBoutenType;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextDisplayAlignment(pub i32);
impl TimedTextDisplayAlignment {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextDisplayAlignment {}
impl ::core::clone::Clone for TimedTextDisplayAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextDisplayAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextDisplayAlignment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextDisplayAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextDisplayAlignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextDisplayAlignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextDisplayAlignment;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextFlowDirection(pub i32);
impl TimedTextFlowDirection {
    pub const LeftToRight: Self = Self(0i32);
    pub const RightToLeft: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextFlowDirection {}
impl ::core::clone::Clone for TimedTextFlowDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextFlowDirection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextFlowDirection {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextFlowDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextFlowDirection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextFlowDirection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextFlowDirection;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextFontStyle(pub i32);
impl TimedTextFontStyle {
    pub const Normal: Self = Self(0i32);
    pub const Oblique: Self = Self(1i32);
    pub const Italic: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextFontStyle {}
impl ::core::clone::Clone for TimedTextFontStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextFontStyle {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextFontStyle {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextFontStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextFontStyle").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextFontStyle {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextFontStyle;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextLineAlignment(pub i32);
impl TimedTextLineAlignment {
    pub const Start: Self = Self(0i32);
    pub const End: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextLineAlignment {}
impl ::core::clone::Clone for TimedTextLineAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextLineAlignment {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextLineAlignment {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextLineAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextLineAlignment").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextLineAlignment {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextLineAlignment;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextRubyAlign(pub i32);
impl TimedTextRubyAlign {
    pub const Center: Self = Self(0i32);
    pub const Start: Self = Self(1i32);
    pub const End: Self = Self(2i32);
    pub const SpaceAround: Self = Self(3i32);
    pub const SpaceBetween: Self = Self(4i32);
    pub const WithBase: Self = Self(5i32);
}
impl ::core::marker::Copy for TimedTextRubyAlign {}
impl ::core::clone::Clone for TimedTextRubyAlign {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextRubyAlign {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextRubyAlign {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextRubyAlign {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRubyAlign").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextRubyAlign {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextRubyAlign;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextRubyPosition(pub i32);
impl TimedTextRubyPosition {
    pub const Before: Self = Self(0i32);
    pub const After: Self = Self(1i32);
    pub const Outside: Self = Self(2i32);
}
impl ::core::marker::Copy for TimedTextRubyPosition {}
impl ::core::clone::Clone for TimedTextRubyPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextRubyPosition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextRubyPosition {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextRubyPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRubyPosition").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextRubyPosition {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextRubyPosition;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextRubyReserve(pub i32);
impl TimedTextRubyReserve {
    pub const None: Self = Self(0i32);
    pub const Before: Self = Self(1i32);
    pub const After: Self = Self(2i32);
    pub const Both: Self = Self(3i32);
    pub const Outside: Self = Self(4i32);
}
impl ::core::marker::Copy for TimedTextRubyReserve {}
impl ::core::clone::Clone for TimedTextRubyReserve {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextRubyReserve {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextRubyReserve {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextRubyReserve {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextRubyReserve").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextRubyReserve {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextRubyReserve;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextScrollMode(pub i32);
impl TimedTextScrollMode {
    pub const Popon: Self = Self(0i32);
    pub const Rollup: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextScrollMode {}
impl ::core::clone::Clone for TimedTextScrollMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextScrollMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextScrollMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextScrollMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextScrollMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextScrollMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextScrollMode;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextUnit(pub i32);
impl TimedTextUnit {
    pub const Pixels: Self = Self(0i32);
    pub const Percentage: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextUnit {}
impl ::core::clone::Clone for TimedTextUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextUnit {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextUnit").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextUnit {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextUnit;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextWeight(pub i32);
impl TimedTextWeight {
    pub const Normal: Self = Self(400i32);
    pub const Bold: Self = Self(700i32);
}
impl ::core::marker::Copy for TimedTextWeight {}
impl ::core::clone::Clone for TimedTextWeight {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextWeight {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextWeight {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextWeight {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextWeight").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextWeight {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextWeight;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextWrapping(pub i32);
impl TimedTextWrapping {
    pub const NoWrap: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
}
impl ::core::marker::Copy for TimedTextWrapping {}
impl ::core::clone::Clone for TimedTextWrapping {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextWrapping {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextWrapping {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextWrapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextWrapping").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextWrapping {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextWrapping;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TimedTextWritingMode(pub i32);
impl TimedTextWritingMode {
    pub const LeftRightTopBottom: Self = Self(0i32);
    pub const RightLeftTopBottom: Self = Self(1i32);
    pub const TopBottomRightLeft: Self = Self(2i32);
    pub const TopBottomLeftRight: Self = Self(3i32);
    pub const LeftRight: Self = Self(4i32);
    pub const RightLeft: Self = Self(5i32);
    pub const TopBottom: Self = Self(6i32);
}
impl ::core::marker::Copy for TimedTextWritingMode {}
impl ::core::clone::Clone for TimedTextWritingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TimedTextWritingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TimedTextWritingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TimedTextWritingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimedTextWritingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TimedTextWritingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.TimedTextWritingMode;i4)");
}
#[doc = "*Required features: `\"Media_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VideoStabilizationEffectEnabledChangedReason(pub i32);
impl VideoStabilizationEffectEnabledChangedReason {
    pub const Programmatic: Self = Self(0i32);
    pub const PixelRateTooHigh: Self = Self(1i32);
    pub const RunningSlowly: Self = Self(2i32);
}
impl ::core::marker::Copy for VideoStabilizationEffectEnabledChangedReason {}
impl ::core::clone::Clone for VideoStabilizationEffectEnabledChangedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VideoStabilizationEffectEnabledChangedReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for VideoStabilizationEffectEnabledChangedReason {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VideoStabilizationEffectEnabledChangedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VideoStabilizationEffectEnabledChangedReason").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VideoStabilizationEffectEnabledChangedReason {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Media.Core.VideoStabilizationEffectEnabledChangedReason;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Media_Core\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
pub struct MseTimeRange {
    pub Start: super::super::Foundation::TimeSpan,
    pub End: super::super::Foundation::TimeSpan,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for MseTimeRange {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for MseTimeRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for MseTimeRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MseTimeRange").field("Start", &self.Start).field("End", &self.End).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::windows_core::TypeKind for MseTimeRange {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeType for MseTimeRange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.Core.MseTimeRange;struct(Windows.Foundation.TimeSpan;i8);struct(Windows.Foundation.TimeSpan;i8))");
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for MseTimeRange {
    fn eq(&self, other: &Self) -> bool {
        self.Start == other.Start && self.End == other.End
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for MseTimeRange {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for MseTimeRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Media_Core\"`*"]
pub struct TimedTextDouble {
    pub Value: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextDouble {}
impl ::core::clone::Clone for TimedTextDouble {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextDouble {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextDouble").field("Value", &self.Value).field("Unit", &self.Unit).finish()
    }
}
impl ::windows_core::TypeKind for TimedTextDouble {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for TimedTextDouble {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextDouble;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
}
impl ::core::cmp::PartialEq for TimedTextDouble {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value && self.Unit == other.Unit
    }
}
impl ::core::cmp::Eq for TimedTextDouble {}
impl ::core::default::Default for TimedTextDouble {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Media_Core\"`*"]
pub struct TimedTextPadding {
    pub Before: f64,
    pub After: f64,
    pub Start: f64,
    pub End: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextPadding {}
impl ::core::clone::Clone for TimedTextPadding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextPadding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextPadding").field("Before", &self.Before).field("After", &self.After).field("Start", &self.Start).field("End", &self.End).field("Unit", &self.Unit).finish()
    }
}
impl ::windows_core::TypeKind for TimedTextPadding {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for TimedTextPadding {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextPadding;f8;f8;f8;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
}
impl ::core::cmp::PartialEq for TimedTextPadding {
    fn eq(&self, other: &Self) -> bool {
        self.Before == other.Before && self.After == other.After && self.Start == other.Start && self.End == other.End && self.Unit == other.Unit
    }
}
impl ::core::cmp::Eq for TimedTextPadding {}
impl ::core::default::Default for TimedTextPadding {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Media_Core\"`*"]
pub struct TimedTextPoint {
    pub X: f64,
    pub Y: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextPoint {}
impl ::core::clone::Clone for TimedTextPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextPoint").field("X", &self.X).field("Y", &self.Y).field("Unit", &self.Unit).finish()
    }
}
impl ::windows_core::TypeKind for TimedTextPoint {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for TimedTextPoint {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextPoint;f8;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
}
impl ::core::cmp::PartialEq for TimedTextPoint {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Unit == other.Unit
    }
}
impl ::core::cmp::Eq for TimedTextPoint {}
impl ::core::default::Default for TimedTextPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Media_Core\"`*"]
pub struct TimedTextSize {
    pub Height: f64,
    pub Width: f64,
    pub Unit: TimedTextUnit,
}
impl ::core::marker::Copy for TimedTextSize {}
impl ::core::clone::Clone for TimedTextSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimedTextSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimedTextSize").field("Height", &self.Height).field("Width", &self.Width).field("Unit", &self.Unit).finish()
    }
}
impl ::windows_core::TypeKind for TimedTextSize {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for TimedTextSize {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Media.Core.TimedTextSize;f8;f8;enum(Windows.Media.Core.TimedTextUnit;i4))");
}
impl ::core::cmp::PartialEq for TimedTextSize {
    fn eq(&self, other: &Self) -> bool {
        self.Height == other.Height && self.Width == other.Width && self.Unit == other.Unit
    }
}
impl ::core::cmp::Eq for TimedTextSize {}
impl ::core::default::Default for TimedTextSize {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
